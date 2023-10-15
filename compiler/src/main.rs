mod preprocessor;

fn main() {
    let mut ex = String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nullam mattis nibh id interdum pretium. Nullam vitae sapien mattis nibh dignissim aliquam. Phasellus sit amet accumsan erat, id ultrices mauris. Sed tincidunt vel ante at gravida. Nunc ac justo ex. Nam sed ullamcorper neque, semper aliquam lectus. In tempus, ligula eu euismod consequat, magna ips//um commodo urna, id imperdiet nunc ligula tincidunt diam. 
Donec eget nulla id orci vehicula porttitor. Nulla et arcu nec elit tincidunt faucibus. //Duis sit amet scelerisque ipsum, quis efficitur enim.
asdasdasd");
    preprocessor::delete_single_line_comments(&mut ex);
    println!("{:?}", ex);
}