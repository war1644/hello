//pub trait Display {
//    fn show(&self);
//}
pub struct Display;
impl Display
{
    pub fn show(msg:&str)
    {
        println!(msg);
    }

    pub fn auto_show(msg:&str)
    {
        println!(msg);

    }
}