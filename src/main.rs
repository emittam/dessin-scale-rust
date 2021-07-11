use std::path::PathBuf;
use imageproc::{drawing::{Canvas, draw_filled_rect_mut, draw_line_segment_mut}, rect::Rect};
use structopt::StructOpt;
use image::{DynamicImage, Rgba, imageops::FilterType, io::Reader as ImageReader};
use anyhow::{Context, Result, bail};
use std::ffi::OsString;

#[derive(StructOpt)]
struct Opt {
    #[structopt(short="g", default_value="3", long, help="グリッドの分割数、2~10の間で指定")]
    grid_size: u32,
    #[structopt(short="s", default_value="1.0", long, help="拡大縮小の倍率、0.1~10.0の間で指定")]
    scale: f32,
    #[structopt(short="t", long, help="グリッドの背景を透明にしたファイルを出力")]
    transparent: bool,
    #[structopt(short="o", default_value="", long, parse(from_os_str), help="出力先のPath")]
    output_path: PathBuf,
    #[structopt(name="file_path", parse(from_os_str), help="グリッド画像を作成する元画像Path")]
    file_path: PathBuf
}

fn check_opt(opt: Opt) -> Result<Opt> {
    if opt.grid_size  < 2 {
        bail!("grid_sizeは2以上にしてください")
    }
    if opt.grid_size > 10 {
        bail!("grid_sizeは10以下にしてください")
    }
    if opt.scale < 0.1 {
        bail!("scaleは0.1以上にしてください")
    }
    if opt.scale > 10.0 {
        bail!("scaleは10.0以下にしてください")
    }
    Ok(opt)
}

fn load_image_file(path: &PathBuf) -> Result<DynamicImage> {
    if !path.exists() {
        bail!("指定されたファイルは存在しません");
    }
    ImageReader::open(path)
        .context("指定されたファイルの読み込みに失敗しました")?
        .decode()
        .context("指定されたファイルは画像ファイルではありません")
}

fn draw_grid(image: DynamicImage, grid: u32, scale: f32, transparent: bool) -> Result<DynamicImage> {
    let new_width = (image.width() as f32 * scale).round() as u32;
    let new_height = (image.height() as f32 * scale).round() as u32;

    let mut resized_image = if transparent {
        let mut i = DynamicImage::new_rgba8(new_width, new_height);
        let all_alpher = Rgba([255, 255, 255, 0]);
        draw_filled_rect_mut( &mut i, Rect::at(0, 0).of_size(new_width, new_height), all_alpher);
        i
    } else {
        image.resize(new_width, new_height, FilterType::Lanczos3)
    };

    let red = Rgba([255, 0, 0, 255]);
    let grid_widht = (new_width / grid) as f32;
    let grid_height = (new_height / grid) as f32;

    for i in 1 .. grid {
        draw_line_segment_mut(&mut resized_image, (grid_widht * i as f32, 0.0), (grid_widht * i as f32, new_height as f32), red);
        draw_line_segment_mut(&mut resized_image, (0.0, grid_height * i as f32), (new_width as f32, grid_height * i as f32), red);
    }

    Ok(resized_image)
}

fn get_save_file_path(input_file_path: &PathBuf,output_file_path : &PathBuf) -> Result<PathBuf> {
    let mut o;
    if output_file_path.eq(&PathBuf::from("")) {
        let dir_path = input_file_path.parent().context("パス構成に失敗しました")?;
        o = PathBuf::from(dir_path);

        let mut new_file_name = OsString::default();
        new_file_name.push(input_file_path.file_stem().context("ファイル名が不正です")?);
        new_file_name.push("_grid");

        o.push(new_file_name);
        o.set_extension(input_file_path.extension().context("パス構成に失敗しました")?);
    } else {
        o = PathBuf::from(output_file_path);
    }
    Ok(o)
}


fn main() -> Result<()> {
    let opt = check_opt(Opt::from_args())?;

    let mut image = load_image_file(&opt.file_path)?;
    image = draw_grid(image, opt.grid_size, opt.scale, opt.transparent)?;

    image.save(get_save_file_path(&opt.file_path, &opt.output_path)?)
    .expect("ファイル保存に失敗しました");

    return Ok(());
}
