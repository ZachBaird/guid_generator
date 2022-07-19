use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use colored::Colorize;
use guid_create::GUID;

fn main() {
    // Assign clipboard ctx and init result
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    let mut result: String = String::new();

    let guid = GUID::rand().to_string();
    let guid = guid.split("-"); // shadow variable

    for slice in guid {
        result += slice;
    }

    result = result.to_lowercase();    

    ctx.set_contents(result.to_owned()).unwrap();
    println!("Guid: {result}");
    println!("{}", "Copied to clipboard.".green().bold());
}
