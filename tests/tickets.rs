#[cfg(test)]
mod tests {
    use axum::http::StatusCode;
    use axum_full_course::{
        domain::model::{CreateTicket, Ticket},
        infrastructure::auth::dto::LoginDto,
        ADDR_PORT, ADDR_URL,
    };
    use reqwest::{Client, Response};
    use std::net::SocketAddr;

    static ADDR: SocketAddr = SocketAddr::new(ADDR_URL, ADDR_PORT);

    // FIXME: duplication
    fn print_resp(resp: &Response) {
        println!("{:#?}", resp.headers());
        println!("{:#?}", resp.cookies().collect::<Vec<_>>());
    }

    #[ignore]
    #[tokio::test]
    async fn it_should_not_allow_to_create_ticket_if_not_auth() -> anyhow::Result<()> {
        let client = Client::new();

        let tickets_url = format!("http://{:#?}/tickets", ADDR);

        let create_ticket = CreateTicket {
            title: "demo".to_string(),
        };

        let create_resp: Ticket = client
            .post(tickets_url.clone())
            .json(&create_ticket)
            .send()
            .await?
            .json()
            .await?;

        Ok(())
    }

    #[tokio::test]
    async fn it_should_create_ticket() -> anyhow::Result<()> {
        let client = Client::new();

        let url = format!("http://{:#?}/auth/login", ADDR);

        let resp = client
            .post(url.clone())
            .json(&LoginDto {
                email: "demo".to_string(),
                password: "test".to_string(),
            })
            .send()
            .await?;

        print_resp(&resp);

        let tickets_url = format!("http://{:#?}/tickets", ADDR);

        let create_ticket = CreateTicket {
            title: "demo".to_string(),
        };

        let create_resp: Ticket = client
            .post(tickets_url.clone())
            .json(&create_ticket)
            .send()
            .await?
            .json()
            .await?;

        assert_eq!(
            create_resp,
            Ticket {
                id: 1,
                title: create_ticket.title.clone()
            }
        );

        let after_create_list_resp: Vec<Ticket> =
            client.get(tickets_url.clone()).send().await?.json().await?;

        assert_eq!(
            after_create_list_resp,
            vec![Ticket {
                id: 1,
                title: create_ticket.title.clone()
            }]
        );

        let delete_resp = client
            .delete(format!("{tickets_url}/{}", create_resp.id))
            .send()
            .await?;

        assert_eq!(delete_resp.status(), StatusCode::OK);

        let after_delete_list_resp: Vec<Ticket> =
            client.get(tickets_url.clone()).send().await?.json().await?;

        assert_eq!(after_delete_list_resp, vec![]);

        Ok(())
    }
}
