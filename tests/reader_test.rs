use std::io;
use std::fs;
use std::path::PathBuf;

const BASE_PATH: [&'static str; 2] = [".", "tests"];

fn open_image(name: &str) -> png::Decoder<io::BufReader<fs::File>> {
    let mut path: PathBuf = BASE_PATH.iter().collect();
    path.push("scanlines");
    path.push(name);

    png::Decoder::new(io::BufReader::new(fs::File::open(path).unwrap()))
}

#[test]
fn read_scanlines() {
    let decoder = open_image("checkerboard.png");
    let (info, mut reader) = decoder.read_info().unwrap();
    assert_eq!(info.color_type, png::ColorType::RGB);
    assert_eq!(info.bit_depth, png::BitDepth::Eight);
    assert_eq!(reader.info().interlaced, false);

    // Even
    for _ in 0..16 {
        let (row, interlace) = reader.next_interlaced_row().unwrap().unwrap();
        assert_eq!(interlace, None);
        assert_eq!(row.len(), 128 * 3);
        assert_eq!(&row[0..(16 * 3)], &[0u8; 16 * 3][..]);
        assert_eq!(&row[(16 * 3)..(32 * 3)], &[255u8; 16 * 3][..]);
    }

    // Odd
    for _ in 0..16 {
        let (row, interlace) = reader.next_interlaced_row().unwrap().unwrap();
        assert_eq!(interlace, None);
        assert_eq!(row.len(), 128 * 3);
        assert_eq!(&row[0..(16 * 3)], &[255u8; 16 * 3][..]);
        assert_eq!(&row[(16 * 3)..(32 * 3)], &[0u8; 16 * 3][..]);
    }

    assert_eq!(reader.skip_rows(16).ok(), Some(16));

    // Odd
    for _ in 0..16 {
        let (row, interlace) = reader.next_interlaced_row().unwrap().unwrap();
        assert_eq!(interlace, None);
        assert_eq!(row.len(), 128 * 3);
        assert_eq!(&row[0..(16 * 3)], &[255u8; 16 * 3][..]);
        assert_eq!(&row[(16 * 3)..(32 * 3)], &[0u8; 16 * 3][..]);
    }

    // Attempt to skip remaining rows with u32::MAX
    assert_eq!(reader.skip_rows(std::u32::MAX).ok(), Some(64));
}

#[test]
fn read_scanlines_interlaced() {
    let decoder = open_image("checkerboard_adam7.png");
    let (info, mut reader) = decoder.read_info().unwrap();
    assert_eq!(info.color_type, png::ColorType::RGB);
    assert_eq!(info.bit_depth, png::BitDepth::Eight);
    assert_eq!(reader.info().interlaced, true);

    for i in 0..2 {
        let (row, interlace) = reader.next_interlaced_row().unwrap().unwrap();
        assert_eq!(interlace, Some((1, i, 16)));
        assert_eq!(row.len(), 16 * 3);
        assert_eq!(row, &[
            0, 0, 0, 0, 0, 0,
            255, 255, 255, 255, 255, 255,
            0, 0, 0, 0, 0, 0,
            255, 255, 255, 255, 255, 255,
            0, 0, 0, 0, 0, 0,
            255, 255, 255, 255, 255, 255,
            0, 0, 0, 0, 0, 0,
            255, 255, 255, 255, 255, 255,
        ][..]);
    }

    for i in 0..2 {
        let (row, interlace) = reader.next_interlaced_row().unwrap().unwrap();
        assert_eq!(interlace, Some((1, i + 2, 16)));
        assert_eq!(row.len(), 16 * 3);
        assert_eq!(row, &[
            255, 255, 255, 255, 255, 255,
            0, 0, 0, 0, 0, 0,
            255, 255, 255, 255, 255, 255,
            0, 0, 0, 0, 0, 0,
            255, 255, 255, 255, 255, 255,
            0, 0, 0, 0, 0, 0,
            255, 255, 255, 255, 255, 255,
            0, 0, 0, 0, 0, 0,
        ][..]);
    }

    // Move from 1st pass to 7th pass
    assert_eq!(reader.skip_rows(12 + 16 + 16 + 32 + 32 + 64).ok(), Some(172));

    // Even
    for i in 0..8 {
        let (row, interlace) = reader.next_interlaced_row().unwrap().unwrap();
        assert_eq!(interlace, Some((7, i, 128)));
        assert_eq!(row.len(), 128 * 3);
        assert_eq!(&row[0..(16 * 3)], &[0u8; 16 * 3][..]);
        assert_eq!(&row[(16 * 3)..(32 * 3)], &[255u8; 16 * 3][..]);
    }

    // Odd
    for i in 0..8 {
        let (row, interlace) = reader.next_interlaced_row().unwrap().unwrap();
        assert_eq!(interlace, Some((7, i + 8, 128)));
        assert_eq!(row.len(), 128 * 3);
        assert_eq!(&row[0..(16 * 3)], &[255u8; 16 * 3][..]);
        assert_eq!(&row[(16 * 3)..(32 * 3)], &[0u8; 16 * 3][..]);
    }

    assert_eq!(reader.skip_rows(8).ok(), Some(8));

    // Odd
    for i in 0..8 {
        let (row, interlace) = reader.next_interlaced_row().unwrap().unwrap();
        assert_eq!(interlace, Some((7, i + 24, 128)));
        assert_eq!(row.len(), 128 * 3);
        assert_eq!(&row[0..(16 * 3)], &[255u8; 16 * 3][..]);
        assert_eq!(&row[(16 * 3)..(32 * 3)], &[0u8; 16 * 3][..]);
    }

    // Attempt to skip remaining rows with u32::MAX
    assert_eq!(reader.skip_rows(std::u32::MAX).ok(), Some(32));
}
