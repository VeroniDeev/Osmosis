use hmac::{Hmac, Mac};
use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::{json, to_vec, Value};
use sha2::Sha256;
use url::form_urlencoded;

pub struct Instagram {
    sig_version: String,
    sig_key: String,
    url: String,
}

impl Instagram {
    pub fn new() -> Self {
        Instagram {
            sig_version: String::from("4"),
            sig_key: String::from(
                "e6358aeede676184b9fe702b30f4fd35e71744605e39d2181a34cede076b3c33",
            ),
            url: String::from("https://i.instagram.com/api/v1/users/lookup/"),
        }
    }

    pub async fn start(&mut self, phonenumber: &str) -> Result<(), reqwest::Error> {
        let mut headers = HeaderMap::new();
        headers.insert("Accept-Language", HeaderValue::from_static("en-US"));
        headers.insert(
            "User-Agent",
            HeaderValue::from_static("Instagram 101.0.0.15.120"),
        );
        headers.insert(
            "Content-Type",
            HeaderValue::from_static("application/x-www-form-urlencoded; charset=UTF-8"),
        );
        headers.insert("Accept-Encoding", HeaderValue::from_static("gzip, deflate"));
        headers.insert("X-FB-HTTP-Engine", HeaderValue::from_static("Liger"));
        headers.insert("Connection", HeaderValue::from_static("close"));

        let signature = self.generate_signature(self.generate_data(phonenumber));

        let client = reqwest::Client::new();
        let response = client
            .post(self.url.as_str())
            .body(signature)
            .headers(headers)
            .send()
            .await?;
        let lamb = response.bytes().await.expect("msg");
        let by = String::from_utf8_lossy(&lamb);
        println!("{}", by);
        let decoded_bytes =
            hex::decode(by.replace("\\x", "")).expect("Erreur de décodage hexadécimal");
        println!("{:?}", decoded_bytes);

        // if response.status().is_success() {
        //     let json_response = response.text().await?;
        //     println!("{}", json_response);

        //     let inter: Value = serde_json::from_str(json_response.as_str()).expect("msg");

        //     let transform: Value = serde_json::from_str(&json_response.as_str()).expect("msg");
        // } else {
        //     println!("Erreur : {:?}", response.status());
        //     println!("{}", response.text().await?);
        // }

        Ok(())
    }

    fn generate_data(&self, phonenumber_raw: &str) -> Value {
        let data = json!({
            "login_attempt_count": "0",
            "directly_sign_in": "true",
            "source": "default",
            "q": phonenumber_raw,
            "ig_sig_key_version": self.sig_version
        });
        return data;
    }

    fn generate_signature(&self, data: Value) -> String {
        let mut hmac = Hmac::<Sha256>::new_from_slice(self.sig_key.as_bytes())
            .expect("HMAC can take key of any size");
        if let Ok(json_bytes) = to_vec(&data) {
            hmac.update(json_bytes.as_slice());
        }
        let result = hmac.finalize().into_bytes();
        let result_hex = hex::encode(result);

        let json_deserialize = serde_json::to_string(&data).expect("ck,l;qhsdlfjkhqsujkd");
        let mut encoded_data = form_urlencoded::Serializer::new(String::new())
            .append_pair("", json_deserialize.as_str())
            .finish();

        encoded_data = encoded_data.replace("=", "");
        encoded_data = encoded_data.replace("2C", "2C+");
        encoded_data = encoded_data.replace("3A", "3A+");

        let signature = format!(
            "ig_sig_key_version={}&signed_body={}.{}",
            self.sig_version, result_hex, encoded_data
        );

        return signature;
    }
}
