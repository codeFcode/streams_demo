extern crate reqwest;
extern crate serde_json;

pub mod http_calls 
{
	use serde::{Serialize, Deserialize};

    // Put your server address and port here
	const SERVER_ADDRESS : &str = "localhost";
	const SERVER_PORT : &str = "3000";

	#[derive(Serialize, Deserialize, Debug)]
	pub struct Data {
		pub state: String,
		pub payload: Payload
	}

	impl Data {
    	pub fn default () -> Data {
        	Data { state: "".to_string(), payload: Payload::default() }
    	}
	}

	#[derive(Serialize, Deserialize, Debug)]
	pub struct Payload {
		pub certificate_number : u32,
		pub partner_code : String,
		pub data_keys : String,
		pub date_time: String
	}

	impl Payload {
    	pub fn default () -> Payload {
        	Payload { certificate_number: 0, partner_code: "".to_string(), data_keys: "".to_string(),  date_time: "".to_string() }
    	}
	}

	pub fn create_stream() -> Data {

		let client = reqwest::Client::new();
        let link = format!("{}{}{}{}{}", &"http://", SERVER_ADDRESS, &":", SERVER_PORT, &"/iotaOrigin/create");
 
    	let mut response = client.post(link.as_str())
       							.body(r#""#)
        						.send().expect("Couldn't read the response");

    	let data: Data = response.json().expect("Couldn't parse the response");
    	return data;

	}

	pub fn send_message(data:&Data) -> String {

		let client = reqwest::Client::new();
        let link = format!("{}{}{}{}{}", &"http://", SERVER_ADDRESS, &":", SERVER_PORT, &"/iotaOrigin/update");

    	let mut response = client.put(link.as_str())
    							.json(data)
        						.send().expect("Couldn't read the response");
        match response.status() {
        	reqwest::StatusCode::Created => {
        		let state: String = response.text().expect("Couldn't parse the response");
    			return state.replace("\"", "");
        	},
        	_ => return "Error".to_string(),
        }

	}

	pub fn get_last_message(state: String) -> Payload {

		let client = reqwest::Client::new();
        let link = format!("{}{}{}{}{}", &"http://", SERVER_ADDRESS, &":", SERVER_PORT, &"/iotaOrigin/balance");

    	let mut response = client.get(link.as_str())
    							.body(r#state)
        						.send().expect("Couldn't read the response");

        match response.status() {
        	reqwest::StatusCode::Ok => {
        		let message: Payload = response.json().expect("Couldn't parse the response");
    			println!("Last message: {:?}\n", message);
    			return message;
        	},
        	_ => return Payload::default(),
        }

	}

	pub fn get_messages_history(root: String) {

		let client = reqwest::Client::new();
        let link = format!("{}{}{}{}{}", &"http://", SERVER_ADDRESS, &":", SERVER_PORT, &"/iotaOrigin/history");

    	let mut response = client.get(link.as_str())
    							.body(r#root)
        						.send().expect("Couldn't read the response");

        if response.status() == reqwest::StatusCode::Ok {
       		let messages_list: Vec<Payload> = response.json().expect("Couldn't parse the response");
    		println!("Messages list {:?}\n", messages_list);
       	} else {
       		println!("Error occured, please enter the correct root id");
       	}

	}

}
