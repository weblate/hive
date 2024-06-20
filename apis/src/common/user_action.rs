use shared_types::TournamentId;

use crate::responses::TournamentResponse;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum UserAction {
    Block,
    Challenge,
    Follow,
    Invite(TournamentId),
    Uninvite(TournamentId),
    Message,
    Unblock,
    Unfollow,
    Kick(Box<TournamentResponse>)
}
