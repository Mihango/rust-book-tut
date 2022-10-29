fn main() {
    let width = 30;
    let height = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        //area(width, height)
        area_tuple((width, height))
    );
}

// step 1
fn area(width: u32, height: u32) -> u32 {
    width * height
}

// step 2: refactor to a tuple - works but does not introduce meaning to the data used
fn area_tuple(dimens: (u32, u32)) -> u32 {
    dimens.0 * dimens.1
}

// step 3: refactor to a struct
struct Rectangle {
    width: u32,
    height: u32,
}

fn area_struct(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
