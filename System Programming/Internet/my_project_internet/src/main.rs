use reqwest::blocking::{Client}; // برای استفاده از Client

fn main() {
    // لینک جدید برای دانلود تصویر
    let url = "https://images.pexels.com/photos/29399294/pexels-photo-29399294/free-photo-of-serene-winter-landscape-with-snowy-mountains.jpeg?auto=compress&cs=tinysrgb&w=1260&h=750&dpr=1";
    let save_path = "D:\\rust\\University\\System Programming\\my_project_internet\\serene_winter_landscape.jpg"; // مسیری که فایل ذخیره می‌شود

    // اگر پوشه مقصد وجود نداشته باشد، آن را بسازید
    std::fs::create_dir_all("D:\\rust\\University\\System Programming\\my_project_internet").expect("Error creating folder");

    let client = Client::new();
    let mut response = client.get(url)
        .send()
        .expect("خطا در دانلود فایل");

    let mut out = std::fs::File::create(save_path).expect("خطا در ایجاد فایل");
    std::io::copy(&mut response, &mut out).expect("خطا در کپی کردن داده‌ها");

    println!("فایل با موفقیت دانلود شد!");
}
