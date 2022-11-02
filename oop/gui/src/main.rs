use gui::{Button, Draw, Screen};

struct SelectBox{
    width: u32,
    height: u32,
    options: Vec<String>,

}

impl Draw for SelectBox{
    fn draw(&self) {
        //draw code...
    }
}

fn main() {
    let screen = Screen{
        components: vec![
            Box::new(SelectBox{
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ]
            }),
          Box::new(Button{
              width: 50,
              length: 10,
              label: String::from("Ok")
          })
        ]
    };

    screen.run();
}
