use std::num::Saturating;

fn main() {
    let mut clamped_css_color: Saturating<u8> =
        Saturating(10);

    println!("\nmultiplying");
    for i in 0..10 {
        clamped_css_color *= Saturating(2);
        dbg!(clamped_css_color);
    }

    println!("\nsubtracting");
    for i in 0..10 {
        clamped_css_color -= Saturating(50);
        dbg!(clamped_css_color);
    }

    let mut clamped_css_color_2: u8 = 10;

    println!("\nsaturating_mul");
    for i in 0..10 {
        clamped_css_color_2 =
            clamped_css_color_2.saturating_mul(2);
        dbg!(clamped_css_color_2);
    }
}
