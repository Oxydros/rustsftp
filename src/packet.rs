
// Packet types
pub const SSH_FXP_INIT : u8 = 1;
pub const SSH_FXP_VERSION : u8 = 2;
pub const SSH_FXP_OPEN : u8 = 3;
pub const SSH_FXP_CLOSE : u8 = 4;
pub const SSH_FXP_READ : u8 = 5;
pub const SSH_FXP_WRITE : u8 = 6;
pub const SSH_FXP_LSTAT : u8 = 7;
pub const SSH_FXP_FSTAT : u8 = 8;
pub const SSH_FXP_SETSTAT : u8 = 9;
pub const SSH_FXP_FSETSTAT : u8 = 10;
pub const SSH_FXP_OPENDIR : u8 = 11;
pub const SSH_FXP_READDIR : u8 = 12;
pub const SSH_FXP_REMOVE : u8 = 13;
pub const SSH_FXP_MKDIR : u8 = 14;
pub const SSH_FXP_RMDIR : u8 = 15;
pub const SSH_FXP_REALPATH : u8 = 16;
pub const SSH_FXP_STAT : u8 = 17;
pub const SSH_FXP_RENAME : u8 = 18;
pub const SSH_FXP_READLINK : u8 = 19;
pub const SSH_FXP_LINK : u8 = 21;
pub const SSH_FXP_BLOCK : u8 = 22;
pub const SSH_FXP_UNBLOCK : u8 = 23;
pub const SSH_FXP_STATUS : u8 = 101;
pub const SSH_FXP_HANDLE : u8 = 102;
pub const SSH_FXP_DATA : u8 = 103;
pub const SSH_FXP_NAME : u8 = 104;
pub const SSH_FXP_ATTRS : u8 = 105;
pub const SSH_FXP_EXTENDED : u8 = 200;
pub const SSH_FXP_EXTENDED_REPLY : u8 = 201;

pub struct Packet {
    length: u32,
    ptype: u8,
    request_id: u32
}