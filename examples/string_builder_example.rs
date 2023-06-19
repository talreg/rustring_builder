use string_builder::StringBuilder;

fn main(){
    let mut sample=StringBuilder::new();
    sample.append("hello world");
    println!("{}",sample.to_string());
    println!("{}",sample.len());
    // ntn direct access
    println!("{}",sample.nth(1).unwrap());
    println!("{}",sample.to_string());
    // concat
    let mut sample2=StringBuilder::new();
    sample2.append("what's the frequency, Kenneth?");
    sample.new_line();
    sample+=sample2;        // sample2 is consumed
    println!("{}",sample.to_string());
    // append different types
    sample.append(123);
    println!("{}",sample.to_string());
    sample2=StringBuilder::new_with_value("Hello world");
    sample2.push('!');
    println!("{}",sample2.to_string()); // Hello world!


}