use lettre::transport::smtp::authentication::Credentials;
use lettre::{SmtpTransport, Transport};
mod configuration;

fn main() {
 let user = "baba ali";
 println!("{}", user);
 let file_input = "Welcome, howdy peopel of the rockshire";
 let message = configuration::message_maker(file_input, "baba.ali@flexisaf.com");

 let cred: Credentials = Credentials::new("babaali196@gmail.com".to_string(), "alibaba34@mail".to_string());

 let mailer = SmtpTransport::relay("smtp.gmail.com").unwrap().credentials(cred).build();

 match mailer.send(&message) {
    Ok(_) => println!("Message was delivered successfully at"),
    Err(e) => panic!("Oops, could not send message, {:?}", e),
 }

}
