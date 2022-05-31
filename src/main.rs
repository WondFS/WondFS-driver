extern crate sysctl;
use sysctl::Sysctl;
use std::ffi::CString;

// const TUN_IOC_MAGIC: u8 = 'T' as u8;
// const TUN_IOC_SET_IFF: u8 = 202;
// ioctl!(write tun_set_iff with TUN_IOC_MAGIC, TUN_IOC_SET_IFF; u32);


// ioctl!是一个函数宏
// 上面宏等价于
// pub unsafe fn tun_set_iff(fd: c_int, data: *mut u32) -> Result<c_int> {
//     let res = libc::ioctl(fd, ior!(TUN_IOC_MAGIC, TUN_IOC_SET_IFF, mem::size_of::<u32>()), data);
//     Errno::result(res)
// }
// 具体可以参考https://docs.rs/nix/0.9.0/nix/sys/ioctl/index.html

// const xx_MAGIC
// const xx_NUM
// ioctl!(read MEMGETINFO with MEMGETINFO_MAGIC, MEMGETINFO_NUM; yy);
//展开宏 fn xx(fd: c_int,data: *mut yy)...

struct mtd_oob_buf {
    start: u32,
    length: u32,
	ptr:CString
}

struct erase_info_user {
    start: u32,
	length: u32
}

struct mtd_info_user {
	tp: u8,
	flags: u32,
    size: u32,	/* Total size of the MTD */
	erasesize: u32,
	writesize: u32,
	oobsize: u32,	/* Amount of OOB data per block (e.g. 16) */
	padding: u64	/* Old obsolete field; do not use */
}

const MEMGETINFO_MAGIC: u8 = 'M' as u8;
const MEMGETINFO_NUM: u8 = 1;
ioctl!(read MEMGETINFO with MEMGETINFO_MAGIC, MEMGETINFO_NUM; xx);


const MEMERASE_MAGIC: u8 = 'M' as u8;
const MEMERASE_NUM: u8 = 2;
ioctl!(write MEMERASE with MEMERASE_MAGIC, MEMERASE_NUM; erase_info_user);

const MEMWRITEOOB_MAGIC: u8 = 'M' as u8;
const MEMWRITEOOB_NUM: u8 = 3;
ioctl!(readwrite MEMWRITEOOB with MEMWRITEOOB_MAGIC, MEMWRITEOOB_NUM; mtd_oob_buf);

const MEMREADOOB_MAGIC: u8 = 'M' as u8;
const MEMREADOOB_NUM: u8 = 4;
ioctl!(readwrite MEMREADOOB with MEMREADOOB_MAGIC, MEMREADOOB_NUM; mtd_oob_buf);

fn main() {
    println!("Hello, world!");
}

// https://github.com/torvalds/linux/blob/master/include/uapi/mtd/mtd-abi.h
// struct mtd_oob_buf {
// 	__u32 start;
// 	__u32 length;
// 	unsigned char __user *ptr;
// };

// struct erase_info_user {
// 	__u32 start;
// 	__u32 length;
// };

// struct mtd_info_user {
// 	__u8 type;
// 	__u32 flags;
// 	__u32 size;	/* Total size of the MTD */
// 	__u32 erasesize;
// 	__u32 writesize;
// 	__u32 oobsize;	/* Amount of OOB data per block (e.g. 16) */
// 	__u64 padding;	/* Old obsolete field; do not use */
// };

/* Get basic MTD characteristics info (better to use sysfs) */
// #define MEMGETINFO		_IOR('M', 1, struct mtd_info_user)
// /* Erase segment of MTD */
// #define MEMERASE		_IOW('M', 2, struct erase_info_user)
// /* Write out-of-band data from MTD */
// #define MEMWRITEOOB		_IOWR('M', 3, struct mtd_oob_buf)
// /* Read out-of-band data from MTD */
// #define MEMREADOOB		_IOWR('M', 4, struct mtd_oob_buf)
// /* Lock a chip (for MTD that supports it) */
// #define MEMLOCK			_IOW('M', 5, struct erase_info_user)
// /* Unlock a chip (for MTD that supports it) */
// #define MEMUNLOCK		_IOW('M', 6, struct erase_info_user)
// /* Get the number of different erase regions */
// #define MEMGETREGIONCOUNT	_IOR('M', 7, int)
// /* Get information about the erase region for a specific index */
// #define MEMGETREGIONINFO	_IOWR('M', 8, struct region_info_user)
// /* Get info about OOB modes (e.g., RAW, PLACE, AUTO) - legacy interface */
// #define MEMGETOOBSEL		_IOR('M', 10, struct nand_oobinfo)
// /* Check if an eraseblock is bad */
// #define MEMGETBADBLOCK		_IOW('M', 11, __kernel_loff_t)
// /* Mark an eraseblock as bad */
// #define MEMSETBADBLOCK		_IOW('M', 12, __kernel_loff_t)
// /* Set OTP (One-Time Programmable) mode (factory vs. user) */
// #define OTPSELECT		_IOR('M', 13, int)
// /* Get number of OTP (One-Time Programmable) regions */
// #define OTPGETREGIONCOUNT	_IOW('M', 14, int)
// /* Get all OTP (One-Time Programmable) info about MTD */
// #define OTPGETREGIONINFO	_IOW('M', 15, struct otp_info)
// /* Lock a given range of user data (must be in mode %MTD_FILE_MODE_OTP_USER) */
// #define OTPLOCK			_IOR('M', 16, struct otp_info)
// /* Get ECC layout (deprecated) */
// #define ECCGETLAYOUT		_IOR('M', 17, struct nand_ecclayout_user)
// /* Get statistics about corrected/uncorrected errors */
// #define ECCGETSTATS		_IOR('M', 18, struct mtd_ecc_stats)
// /* Set MTD mode on a per-file-descriptor basis (see "MTD file modes") */
// #define MTDFILEMODE		_IO('M', 19)
// /* Erase segment of MTD (supports 64-bit address) */
// #define MEMERASE64		_IOW('M', 20, struct erase_info_user64)
// /* Write data to OOB (64-bit version) */
// #define MEMWRITEOOB64		_IOWR('M', 21, struct mtd_oob_buf64)
// /* Read data from OOB (64-bit version) */
// #define MEMREADOOB64		_IOWR('M', 22, struct mtd_oob_buf64)
// /* Check if chip is locked (for MTD that supports it) */
// #define MEMISLOCKED		_IOR('M', 23, struct erase_info_user)