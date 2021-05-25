extern crate reqwest;
extern crate serde_json;

use postgres::{Client, NoTls};
use serde_json::json;
use std::io::{self};
use chrono::prelude::*;

mod http_calls;

fn main() {

	println!("---------------------------------------------------------------------------------------------");
	println!("------------------------- Welcome in the Iota streams client demo! --------------------------");
	println!("---------------------------------------------------------------------------------------------");
	let mut finished = false;
	let mut buffer = String::new();
	let stdin = io::stdin();
	let mut ok;
	let mut trimmed;
	let mut state;
	let mut data : http_calls::http_calls::Data = http_calls::http_calls::Data::default();
  let mut partner_code;

	while !finished {

		println!("Choose one operation:\n1- Create a new stream\n2- Add a message to a stream");
		println!("3- Get the last message in a stream\n4- Get messages history of a stream");
		stdin.read_line(&mut buffer).expect("Cannot read value");
  	trimmed = buffer.trim();
  	match trimmed.parse::<u32>() {
     		Ok(value) => { 
     			match value {
     		 		1 => { 
     		 			// We create a stream
					    data = http_calls::http_calls::create_stream();
					    println!("Here is the root id of your stream: {}", data.state);
       		 	},

       		 	2 => {

       		 		println!("You choose to add a certificate message to your stream\nEnter the last state of your stream");
     		 			buffer = String::new();
     		 			stdin.read_line(&mut buffer).expect("Cannot read value");
     		 			state = buffer.trim();
              data.state = state.to_string();

              println!("Enter the number of certificates of the transfer");
              buffer = String::new();
              stdin.read_line(&mut buffer).expect("Cannot read value");
              trimmed = buffer.trim();

              match trimmed.parse::<u32>() {
                Ok(certificate_number) => {
                    println!("Enter the partner code");
                    buffer = String::new();
                    stdin.read_line(&mut buffer).expect("Cannot read value");
                    partner_code = buffer.trim();
                    // We construct the payload of the message
                    data.payload = { http_calls::http_calls::Payload 
                      {
                        certificate_number: certificate_number,
                        partner_code: partner_code.to_string(),
                        data_keys: String::from(""),
                        date_time: Local::now().naive_utc().to_string().replace(" ", "T")
                      }
                    };
                    // Now we can add the messge in the given stream
                    let updated_state = http_calls::http_calls::send_message(&data);
                    if updated_state == "Error" {
                      println!("Error occured, please enter the correct root id")
                    } else {
                     println!("Message added successfully! Here is the updated state of your stream: {} ", updated_state);
                    }
                },
                Err(..) => {  println!("Not a number!");},
              };
       		 	},

     		 		3 => {
              println!("Enter the last state of your stream");
              buffer = String::new();
              stdin.read_line(&mut buffer).expect("Cannot read value");
              state = buffer.trim();

              //Get the last certificate message
              let payload = http_calls::http_calls::get_last_message(json!(state).to_string());
              if payload.date_time == "" {
                println!("Error occured, please enter the correct last state")
              }
            },

       	 		4 => {
              println!("Enter the root id of your stream");
              buffer = String::new();
              stdin.read_line(&mut buffer).expect("Cannot read value");
              state = buffer.trim();

              //Get the history of certifictate messages
              http_calls::http_calls::get_messages_history(json!(state).to_string());
            },

     		 		_ => { println!("Choice not in the provided list!") }

    			}
 			},
     		Err(..) => println!("Choice not in the provided list!"),
   	};

		ok = false;
		buffer = String::new();
    	while !ok {
    		println!("Another operation?\n1-Yes\n2-No");
    		stdin.read_line(&mut buffer).expect("Cannot read value");
    		trimmed = buffer.trim();
    		match trimmed.parse::<u32>() {
       			Ok(value) => { 
       				match value {
       			 		1 => { finished = false; ok = true },
       			 		2 => { finished = true; ok = true },
       			 		_ => { ok = false; println!("Please, choose 1 or 2") }
       				}
       			},
        		Err(..) => println!("Please, enter a number"),
   			};
   			buffer = String::new(); 
    	}

	}
    
}

// If you want to store your streams informations in a postgres database
#[allow(dead_code)]
fn save_info(state: &String) {
	let mut client = Client::connect("postgresql://postgres@localhost:5432/streams", 
                                    NoTls).expect("Can't connect to the database");
	let info = StreamInfo {
            state: String::from(state),
            root: String::from(state)
    };

    client.batch_execute(r#"CREATE TABLE IF NOT EXISTS streams (
        	id         SERIAL PRIMARY KEY,
        	root       TEXT,
        	state      TEXT
    	)"#
    ).expect("Can't create table");

    client.execute(
                r#"INSERT INTO streams (root, state) VALUES ($1, $2)"#,
                &[&info.root, &info.state],
        ).expect("Error appeared while inserting data in the database");

}

pub struct StreamInfo {
		pub state : String,
		pub root: String
}

/*#[cfg(test)]
mod WebappTest
{
	mod http_calls;
	#[test]
	fn test_post_message_ok() {

		let client = reqwest::Client::new();
		let response = http_calls::http_calls::post(client).unwrap();
		assert!(response.status() == reqwest::StatusCode::Ok);
	}

}*/

