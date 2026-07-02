use tokio::net::TcpListener;

use crate::{judge::judge::ServerJudge, profile::Profile};

const SERVER_PORT : u32 = 7212011;

struct Server<'a>
{
    pub judges: Vec<ServerJudge<'a>>,
    pub profile: Profile<'a>,
    pub listener: TcpListener,
    
}

