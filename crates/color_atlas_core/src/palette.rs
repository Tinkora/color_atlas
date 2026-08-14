use crate::color::{Color, hsl_to_rgb, rgb_to_hsl};
use crate::error::CoreError;

/// Maximum number of colors returned by one palette request.
pub const MAX_PALETTE_COLORS: u32 = 20;
/// Maximum number of source pixels accepted by the core API.
pub const MAX_IMAGE_PIXELS: usize = 16_777_216;
const MAX_SAMPLES: usize = 10_000;

// ---------------------------------------------------------------------------
// k-means palette extraction
// ---------------------------------------------------------------------------

/// Extract a palette of `count` dominant colors from raw RGBA pixel data using k-means clustering.
///
/// The algorithm:
/// 1. Downsample large images for performance.
/// 2. Initialize centroids using k-means++ (spread them apart).
/// 3. Iterate assignment + update until convergence (max 20 iterations).
///
/// `rgba_pixels` is a flat slice of RGBA bytes (4 bytes per pixel).
///
/// Returns `count` `Color` values representing the dominant colors.
pub fn extract_palette(
    rgba_pixels: &[u8],
    width: u32,
    height: u32,
    count: u32,
) -> Result<Vec<Color>, CoreError> {
    if count == 0 {
        return Err(CoreError::ZeroCount(count));
    }

    if count > MAX_PALETTE_COLORS {
        return Err(CoreError::PaletteCountTooLarge {
            requested: count,
            max: MAX_PALETTE_COLORS,
        });
    }

    let pixel_count =
        (width as usize)
            .checked_mul(height as usize)
            .ok_or(CoreError::ImageTooLarge {
                pixels: usize::MAX,
                max: MAX_IMAGE_PIXELS,
            })?;
    if pixel_count == 0 {
        return Err(CoreError::EmptyPixels);
    }
    if pixel_count > MAX_IMAGE_PIXELS {
        return Err(CoreError::ImageTooLarge {
            pixels: pixel_count,
            max: MAX_IMAGE_PIXELS,
        });
    }

    let expected_bytes = pixel_count.checked_mul(4).ok_or(CoreError::ImageTooLarge {
        pixels: pixel_count,
        max: MAX_IMAGE_PIXELS,
    })?;
    if rgba_pixels.len() != expected_bytes {
        return Err(CoreError::InvalidPixelBuffer {
            expected: expected_bytes,
            actual: rgba_pixels.len(),
        });
    }

    // Sample pixels for performance while keeping the sample bound explicit.
    let step = if pixel_count > MAX_SAMPLES {
        pixel_count.div_ceil(MAX_SAMPLES)
    } else {
        1
    };

    let samples: Vec<[f64; 3]> = rgba_pixels
        .chunks_exact(4)
        .step_by(step)
        .filter(|chunk| chunk[3] != 0)
        .map(|chunk| [chunk[0] as f64, chunk[1] as f64, chunk[2] as f64])
        .collect();

    if samples.is_empty() {
        return Err(CoreError::EmptyPixels);
    }

    let k = (count as usize).min(samples.len());

    // k-means++ initialization
    let mut rng: u64 = 42;
    let mut centroids: Vec<[f64; 3]> = Vec::with_capacity(k);

    // Choose first centroid randomly
    let first_idx = (next_rand(&mut rng) as usize) % samples.len();
    centroids.push(samples[first_idx]);

    // Choose remaining centroids with probability proportional to squared distance
    while centroids.len() < k {
        let distances: Vec<f64> = samples
            .iter()
            .map(|p| {
                centroids
                    .iter()
                    .map(|c| sq_dist(p, c))
                    .fold(f64::MAX, f64::min)
            })
            .collect();

        let sum: f64 = distances.iter().sum();
        if sum == 0.0 {
            // All remaining points are identical to existing centroids
            break;
        }

        let threshold = next_rand_f64(&mut rng) * sum;
        let mut cumulative = 0.0;
        let mut chosen = 0;
        for (i, &d) in distances.iter().enumerate() {
            cumulative += d;
            if cumulative >= threshold {
                chosen = i;
                break;
            }
        }

        centroids.push(samples[chosen]);
    }

    // Lloyd's algorithm
    let max_iters = 20;
    for _ in 0..max_iters {
        // Assignment: assign each sample to nearest centroid
        let mut assignments: Vec<usize> = Vec::with_capacity(samples.len());
        for s in &samples {
            let mut best = 0;
            let mut best_dist = f64::MAX;
            for (j, c) in centroids.iter().enumerate() {
                let d = sq_dist(s, c);
                if d < best_dist {
                    best_dist = d;
                    best = j;
                }
            }
            assignments.push(best);
        }

        // Update: move centroids to mean of assigned points
        let mut new_centroids: Vec<[f64; 3]> = vec![[0.0; 3]; centroids.len()];
        let mut counts: Vec<usize> = vec![0; centroids.len()];

        for (i, s) in samples.iter().enumerate() {
            let c = assignments[i];
            new_centroids[c][0] += s[0];
            new_centroids[c][1] += s[1];
            new_centroids[c][2] += s[2];
            counts[c] += 1;
        }

        let mut converged = true;
        for (j, nc) in new_centroids.iter_mut().enumerate() {
            if counts[j] > 0 {
                nc[0] /= counts[j] as f64;
                nc[1] /= counts[j] as f64;
                nc[2] /= counts[j] as f64;
            }
            if sq_dist(nc, &centroids[j]) > 0.01 {
                converged = false;
            }
        }

        centroids = new_centroids;

        if converged {
            break;
        }
    }

    // Sort centroids by perceived brightness (relative luminance estimate) for stable output
    centroids.sort_by(|a, b| {
        let la = 0.299 * a[0] + 0.587 * a[1] + 0.114 * a[2];
        let lb = 0.299 * b[0] + 0.587 * b[1] + 0.114 * b[2];
        la.partial_cmp(&lb).unwrap_or(std::cmp::Ordering::Equal)
    });

    Ok(centroids
        .iter()
        .take(count as usize)
        .map(|c| {
            Color::rgb(
                c[0].round().clamp(0.0, 255.0) as u8,
                c[1].round().clamp(0.0, 255.0) as u8,
                c[2].round().clamp(0.0, 255.0) as u8,
            )
        })
        .collect())
}

// ---------------------------------------------------------------------------
// Color Harmony
// ---------------------------------------------------------------------------

/// Return the complementary color (180° shift on the hue wheel).
pub fn complementary(color: &Color) -> Color {
    let (h, s, l) = rgb_to_hsl(color.r, color.g, color.b);
    hsl_to_rgb((h + 180.0) % 360.0, s, l)
}

/// Return 5 analogous colors centered on the given color (±30°, ±60°).
pub fn analogous(color: &Color) -> [Color; 5] {
    let (h, s, l) = rgb_to_hsl(color.r, color.g, color.b);
    [
        hsl_to_rgb((h + 360.0 - 60.0) % 360.0, s, l),
        hsl_to_rgb((h + 360.0 - 30.0) % 360.0, s, l),
        *color,
        hsl_to_rgb((h + 30.0) % 360.0, s, l),
        hsl_to_rgb((h + 60.0) % 360.0, s, l),
    ]
}

/// Return the triadic colors (0°, 120°, 240° shift).
pub fn triadic(color: &Color) -> [Color; 3] {
    let (h, s, l) = rgb_to_hsl(color.r, color.g, color.b);
    [
        *color,
        hsl_to_rgb((h + 120.0) % 360.0, s, l),
        hsl_to_rgb((h + 240.0) % 360.0, s, l),
    ]
}

/// Return the tetradic (rectangular) colors: 0°, 60°, 180°, 240°.
pub fn tetradic(color: &Color) -> [Color; 4] {
    let (h, s, l) = rgb_to_hsl(color.r, color.g, color.b);
    [
        *color,
        hsl_to_rgb((h + 60.0) % 360.0, s, l),
        hsl_to_rgb((h + 180.0) % 360.0, s, l),
        hsl_to_rgb((h + 240.0) % 360.0, s, l),
    ]
}

/// Return `count` monochromatic variations by varying lightness.
pub fn monochromatic(color: &Color, count: u32) -> Vec<Color> {
    if count == 0 {
        return vec![];
    }
    let (h, s, _l) = rgb_to_hsl(color.r, color.g, color.b);
    (0..count)
        .map(|i| {
            let t = if count == 1 {
                0.5
            } else {
                i as f64 / (count - 1) as f64
            };
            let new_l = (t * 0.9 + 0.05).clamp(0.05, 0.95);
            hsl_to_rgb(h, s, new_l)
        })
        .collect()
}

/// Return `count` progressively darker shades (reduce lightness toward 0).
pub fn shades(color: &Color, count: u32) -> Vec<Color> {
    if count == 0 {
        return vec![];
    }
    let (h, s, l) = rgb_to_hsl(color.r, color.g, color.b);
    (0..count)
        .map(|i| {
            let t = if count == 1 {
                1.0
            } else {
                1.0 - (i as f64 / (count - 1) as f64)
            };
            let new_l = (l * t).max(0.0);
            hsl_to_rgb(h, s, new_l)
        })
        .collect()
}

/// Return `count` progressively lighter tints (increase lightness toward 1.0).
pub fn tints(color: &Color, count: u32) -> Vec<Color> {
    if count == 0 {
        return vec![];
    }
    let (h, s, l) = rgb_to_hsl(color.r, color.g, color.b);
    (0..count)
        .map(|i| {
            let t = if count == 1 {
                0.0
            } else {
                i as f64 / (count - 1) as f64
            };
            let new_l = (l + (1.0 - l) * t).min(1.0);
            hsl_to_rgb(h, s, new_l)
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Color blindness simulation
// ---------------------------------------------------------------------------

/// Simulate protanopia (red-blind).
pub fn simulate_protanopia(color: &Color) -> Color {
    apply_cvd_matrix(color, &PROTANOPIA_MATRIX)
}

/// Simulate deuteranopia (green-blind).
pub fn simulate_deuteranopia(color: &Color) -> Color {
    apply_cvd_matrix(color, &DEUTERANOPIA_MATRIX)
}

/// Simulate tritanopia (blue-blind).
pub fn simulate_tritanopia(color: &Color) -> Color {
    apply_cvd_matrix(color, &TRITANOPIA_MATRIX)
}

/// Lightweight linear sRGB approximations for color-vision-deficiency previews.
/// These results are for design review only; they are not a clinical simulation.
type CvdMatrix = [[f64; 3]; 3];

const PROTANOPIA_MATRIX: CvdMatrix = [
    [0.56667, 0.43333, 0.0],
    [0.55833, 0.44167, 0.0],
    [0.0, 0.24167, 0.75833],
];

const DEUTERANOPIA_MATRIX: CvdMatrix = [[0.625, 0.375, 0.0], [0.7, 0.3, 0.0], [0.0, 0.3, 0.7]];

const TRITANOPIA_MATRIX: CvdMatrix = [
    [0.95, 0.05, 0.0],
    [0.0, 0.43333, 0.56667],
    [0.0, 0.475, 0.525],
];

fn apply_cvd_matrix(color: &Color, matrix: &CvdMatrix) -> Color {
    let r = color.r as f64;
    let g = color.g as f64;
    let b = color.b as f64;

    let nr = matrix[0][0] * r + matrix[0][1] * g + matrix[0][2] * b;
    let ng = matrix[1][0] * r + matrix[1][1] * g + matrix[1][2] * b;
    let nb = matrix[2][0] * r + matrix[2][1] * g + matrix[2][2] * b;

    Color::rgb(
        nr.round().clamp(0.0, 255.0) as u8,
        ng.round().clamp(0.0, 255.0) as u8,
        nb.round().clamp(0.0, 255.0) as u8,
    )
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn sq_dist(a: &[f64; 3], b: &[f64; 3]) -> f64 {
    let dr = a[0] - b[0];
    let dg = a[1] - b[1];
    let db = a[2] - b[2];
    dr * dr + dg * dg + db * db
}

/// Simple xorshift64 PRNG for k-means++ (deterministic seed for reproducibility).
fn next_rand(state: &mut u64) -> u64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state
}

fn next_rand_f64(state: &mut u64) -> f64 {
    (next_rand(state) as f64) / (u64::MAX as f64)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn complementary_works() {
        let c = Color::rgb(255, 0, 0);
        let comp = complementary(&c);
        // Red's complementary is cyan ~ (0, 255, 255)
        assert!(comp.g > 200);
        assert!(comp.b > 200);
    }

    #[test]
    fn triadic_has_three() {
        let c = Color::rgb(255, 0, 0);
        let t = triadic(&c);
        assert_eq!(t.len(), 3);
    }

    #[test]
    fn tetradic_has_four() {
        let c = Color::rgb(255, 0, 0);
        let t = tetradic(&c);
        assert_eq!(t.len(), 4);
    }

    #[test]
    fn analogous_has_five() {
        let c = Color::rgb(120, 200, 80);
        let a = analogous(&c);
        assert_eq!(a.len(), 5);
    }

    #[test]
    fn monochromatic_count() {
        let c = Color::rgb(100, 150, 200);
        let m = monochromatic(&c, 5);
        assert_eq!(m.len(), 5);
    }

    #[test]
    fn shades_count() {
        let c = Color::rgb(200, 100, 50);
        let s = shades(&c, 3);
        assert_eq!(s.len(), 3);
        // First shade should be original, subsequent darker
        assert!(s[0].r >= s[1].r);
    }

    #[test]
    fn tints_count() {
        let c = Color::rgb(200, 100, 50);
        let t = tints(&c, 3);
        assert_eq!(t.len(), 3);
        // First tint should be original, subsequent lighter
        assert!(t[0].r <= t[1].r);
    }

    #[test]
    fn extract_palette_basic() {
        // Simple 2x2 image: red, green, blue, white
        let pixels = vec![
            255, 0, 0, 255, // red
            0, 255, 0, 255, // green
            0, 0, 255, 255, // blue
            255, 255, 255, 255, // white
        ];
        let palette = extract_palette(&pixels, 2, 2, 3).unwrap();
        assert_eq!(palette.len(), 3);
    }

    #[test]
    fn extract_palette_empty_fails() {
        assert!(extract_palette(&[], 0, 0, 3).is_err());
    }

    #[test]
    fn extract_palette_zero_count_fails() {
        let pixels = vec![255, 0, 0, 255];
        assert!(extract_palette(&pixels, 1, 1, 0).is_err());
    }

    #[test]
    fn extract_palette_rejects_oversized_count() {
        let pixels = vec![255, 0, 0, 255];
        let error = extract_palette(&pixels, 1, 1, MAX_PALETTE_COLORS + 1).unwrap_err();
        assert!(matches!(error, CoreError::PaletteCountTooLarge { .. }));
    }

    #[test]
    fn extract_palette_rejects_short_pixel_buffer() {
        let error = extract_palette(&[255, 0, 0, 255], 2, 1, 1).unwrap_err();
        assert!(matches!(error, CoreError::InvalidPixelBuffer { .. }));
    }

    #[test]
    fn extract_palette_rejects_trailing_pixel_data() {
        let pixels = vec![
            255, 0, 0, 255, // declared pixel
            0, 0, 255, 255, // data outside the declared dimensions
        ];
        let error = extract_palette(&pixels, 1, 1, 1).unwrap_err();
        assert!(matches!(
            error,
            CoreError::InvalidPixelBuffer {
                expected: 4,
                actual: 8
            }
        ));
    }

    #[test]
    fn extract_palette_ignores_fully_transparent_pixels() {
        let pixels = vec![
            255, 0, 0, 0, // transparent red
            0, 0, 255, 255, // opaque blue
        ];
        let palette = extract_palette(&pixels, 2, 1, 1).unwrap();
        assert_eq!(palette, vec![Color::rgb(0, 0, 255)]);
    }

    #[test]
    fn extract_palette_does_not_pad_tiny_images_with_black() {
        let palette = extract_palette(&[12, 34, 56, 255], 1, 1, 3).unwrap();
        assert_eq!(palette, vec![Color::rgb(12, 34, 56)]);
    }

    #[test]
    fn cvd_simulation_does_not_panic() {
        let c = Color::rgb(255, 100, 50);
        let _ = simulate_protanopia(&c);
        let _ = simulate_deuteranopia(&c);
        let _ = simulate_tritanopia(&c);
    }
}
