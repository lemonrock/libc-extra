// This file is part of libc-extra. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT. No part of libc-extra, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of libc-extra. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT.



pub const ETH_RX_NFC_IP4: c_uchar = 1;

pub const ETHTOOL_RX_FLOW_SPEC_RING: c_uint = 4294967295;
pub const ETHTOOL_RX_FLOW_SPEC_RING_VF: c_ulonglong = 1095216660480;
pub const ETHTOOL_RX_FLOW_SPEC_RING_VF_OFF: c_uchar = 32;

pub const ETH_RXFH_INDIR_NO_CHANGE: c_uint = 4294967295;

pub const ETHTOOL_RXNTUPLE_ACTION_DROP: c_char = -1;
pub const ETHTOOL_RXNTUPLE_ACTION_CLEAR: c_char = -2;


pub const ETH_FW_DUMP_DISABLE: c_uchar = 0;

pub const ETHTOOL_GSET: c_uchar = 1;
pub const ETHTOOL_SSET: c_uchar = 2;
pub const ETHTOOL_GDRVINFO: c_uchar = 3;
pub const ETHTOOL_GREGS: c_uchar = 4;
pub const ETHTOOL_GWOL: c_uchar = 5;
pub const ETHTOOL_SWOL: c_uchar = 6;
pub const ETHTOOL_GMSGLVL: c_uchar = 7;
pub const ETHTOOL_SMSGLVL: c_uchar = 8;
pub const ETHTOOL_NWAY_RST: c_uchar = 9;
pub const ETHTOOL_GLINK: c_uchar = 10;
pub const ETHTOOL_GEEPROM: c_uchar = 11;
pub const ETHTOOL_SEEPROM: c_uchar = 12;
pub const ETHTOOL_GCOALESCE: c_uchar = 14;
pub const ETHTOOL_SCOALESCE: c_uchar = 15;
pub const ETHTOOL_GRINGPARAM: c_uchar = 16;
pub const ETHTOOL_SRINGPARAM: c_uchar = 17;
pub const ETHTOOL_GPAUSEPARAM: c_uchar = 18;
pub const ETHTOOL_SPAUSEPARAM: c_uchar = 19;
pub const ETHTOOL_GRXCSUM: c_uchar = 20;
pub const ETHTOOL_SRXCSUM: c_uchar = 21;
pub const ETHTOOL_GTXCSUM: c_uchar = 22;
pub const ETHTOOL_STXCSUM: c_uchar = 23;
pub const ETHTOOL_GSG: c_uchar = 24;
pub const ETHTOOL_SSG: c_uchar = 25;
pub const ETHTOOL_TEST: c_uchar = 26;
pub const ETHTOOL_GSTRINGS: c_uchar = 27;
pub const ETHTOOL_PHYS_ID: c_uchar = 28;
pub const ETHTOOL_GSTATS: c_uchar = 29;
pub const ETHTOOL_GTSO: c_uchar = 30;
pub const ETHTOOL_STSO: c_uchar = 31;
pub const ETHTOOL_GPERMADDR: c_uchar = 32;
pub const ETHTOOL_GUFO: c_uchar = 33;
pub const ETHTOOL_SUFO: c_uchar = 34;
pub const ETHTOOL_GGSO: c_uchar = 35;
pub const ETHTOOL_SGSO: c_uchar = 36;
pub const ETHTOOL_GFLAGS: c_uchar = 37;
pub const ETHTOOL_SFLAGS: c_uchar = 38;
pub const ETHTOOL_GPFLAGS: c_uchar = 39;
pub const ETHTOOL_SPFLAGS: c_uchar = 40;
pub const ETHTOOL_GRXFH: c_uchar = 41;
pub const ETHTOOL_SRXFH: c_uchar = 42;
pub const ETHTOOL_GGRO: c_uchar = 43;
pub const ETHTOOL_SGRO: c_uchar = 44;
pub const ETHTOOL_GRXRINGS: c_uchar = 45;
pub const ETHTOOL_GRXCLSRLCNT: c_uchar = 46;
pub const ETHTOOL_GRXCLSRULE: c_uchar = 47;
pub const ETHTOOL_GRXCLSRLALL: c_uchar = 48;
pub const ETHTOOL_SRXCLSRLDEL: c_uchar = 49;
pub const ETHTOOL_SRXCLSRLINS: c_uchar = 50;
pub const ETHTOOL_FLASHDEV: c_uchar = 51;
pub const ETHTOOL_RESET: c_uchar = 52;
pub const ETHTOOL_SRXNTUPLE: c_uchar = 53;
pub const ETHTOOL_GRXNTUPLE: c_uchar = 54;
pub const ETHTOOL_GSSET_INFO: c_uchar = 55;
pub const ETHTOOL_GRXFHINDIR: c_uchar = 56;
pub const ETHTOOL_SRXFHINDIR: c_uchar = 57;
pub const ETHTOOL_GFEATURES: c_uchar = 58;
pub const ETHTOOL_SFEATURES: c_uchar = 59;
pub const ETHTOOL_GCHANNELS: c_uchar = 60;
pub const ETHTOOL_SCHANNELS: c_uchar = 61;
pub const ETHTOOL_SET_DUMP: c_uchar = 62;
pub const ETHTOOL_GET_DUMP_FLAG: c_uchar = 63;
pub const ETHTOOL_GET_DUMP_DATA: c_uchar = 64;
pub const ETHTOOL_GET_TS_INFO: c_uchar = 65;
pub const ETHTOOL_GMODULEINFO: c_uchar = 66;
pub const ETHTOOL_GMODULEEEPROM: c_uchar = 67;

pub const ETHTOOL_GEEE: c_uchar = 68;
pub const ETHTOOL_SEEE: c_uchar = 69;

pub const ETHTOOL_GRSSH: c_uchar = 70;
pub const ETHTOOL_SRSSH: c_uchar = 71;

pub const ETHTOOL_GTUNABLE: c_uchar = 72;
pub const ETHTOOL_STUNABLE: c_uchar = 73;

pub const SPARC_ETH_GSET: c_uchar = 1;
pub const SPARC_ETH_SSET: c_uchar = 2;

pub const DUPLEX_HALF: c_uchar = 0;
pub const DUPLEX_FULL: c_uchar = 1;
pub const DUPLEX_UNKNOWN: c_uchar = 255;

pub const AUTONEG_DISABLE: c_uchar = 0;
pub const AUTONEG_ENABLE: c_uchar = 1;

pub const FLOW_EXT: c_uint = 2147483648;
pub const FLOW_MAC_EXT: c_uint = 1073741824;

pub const ETH_RESET_SHARED_SHIFT: c_uchar = 16;
