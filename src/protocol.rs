// DONET SOFTWARE
// Copyright (c) 2023, Donet Authors.
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License version 3.
// You should have received a copy of this license along
// with this source code in a file named "LICENSE."
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program; if not, write to the Free Software Foundation,
// Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.

use strum_macros::EnumIter;

#[repr(u16)] // 16-bit alignment
#[derive(Copy, Clone, EnumIter)]
pub enum Message {
    ClientHello = 1,
    ClientHelloResp = 2,
    // Sent by the client when it's leaving.
    ClientDisconnect = 3,
    // Sent by the server when it decides to force drop the client.
    ClientEject = 4,
    ClientHeartbeat = 5,

    ClientObjectSetField = 120,
    ClientObjectSetFields = 121,
    ClientObjectLeaving = 132,
    ClientObjectLeavingOwner = 161,
    ClientEnterObjectRequired = 142,
    ClientEnterObjectRequiredOther = 143,
    ClientEnterObjectRequiredOwner = 172,
    ClientEnterObjectRequiredOwnerOther = 173,

    ClientDoneInterestResp = 204,

    ClientAddInterest = 200,
    ClientAddInterestMultiple = 201,
    ClientRemoveInterest = 203,
    ClientObjectLocation = 140,

    // ---------- Internal Messages ---------- //
    // Client Agent
    CASetState = 1000,
    CASetClientID = 1001,
    CASendDatagram = 1002,
    CAEject = 1004,
    CADrop = 1005,
    CAGetNetworkAddress = 1006,
    CAGetNetworkAddressResp = 1007,
    CADeclareObject = 1010,
    CAUndeclareObject = 1011,
    CAAddSessionObject = 1012,
    CARemoveSessionObject = 1013,
    CASetFieldsSendable = 1014,
    CAOpenChannel = 1100,
    CACloseChannel = 1101,
    CAAddPostRemove = 1110,
    CAClearPostRemoves = 1111,
    CAAddInterest = 1200,
    CAAddInterestMultiple = 1201,
    CARemoveInterest = 1203,

    // State Server
    SSCreateObjectWithRequired = 2000,
    SSCreateObjectWithRequiredOther = 2001,
    SSDeleteAIObjects = 2009,
    SSObjectGetField = 2010,
    SSObjectGetFieldResp = 2011,
    SSObjectGetFields = 2012,
    SSObjectGetFieldsResp = 2013,
    SSObjectGetAll = 2014,
    SSObjectGetAllResp = 2015,
    SSObjectSetField = 2020,
    SSObjectSetFields = 2021,
    SSObjectDeleteFieldRAM = 2030,
    SSObjectDeleteFieldsRAM = 2031,
    SSObjectDeleteRAM = 2032,
    SSObjectSetLocation = 2040,
    SSObjectChangingLocation = 2041,
    SSObjectEnterLocationWithRequired = 2042,
    SSObjectEnterLocationWithRequiredOther = 2043,
    SSObjectGetLocation = 2044,
    SSObjectGetLocationResp = 2045,
    SSObjectSetAI = 2050,
    SSObjectChangingAI = 2051,
    SSObjectEnterAIWithRequired = 2052,
    SSObjectEnterAIWithRequiredOther = 2053,
    SSObjectGetAI = 2054,
    SSObjectGetAIResp = 2055,
    SSObjectSetOwner = 2060,
    SSObjectChangingOwner = 2061,
    SSObjectEnterOwnerWithRequired = 2062,
    SSObjectEnterOwnerWithRequiredOther = 2063,
    SSObjectGetOwner = 2064,
    SSObjectGetOwnerResp = 2065,
    SSObjectGetZoneObjects = 2100,
    SSObjectGetZonesObjects = 2102,
    SSObjectGetChildren = 2104,
    SSObjectGetZoneCount = 2110,
    SSObjectGetZoneCountResp = 2111,
    SSObjectGetZonesCount = 2112,
    SSObjectGetZonesCountResp = 2113,
    SSObjectGetChildCount = 2114,
    SSObjectGetChildCountResp = 2115,
    SSObjectDeleteZone = 2120,
    SSObjectDeleteZones = 2122,
    SSObjectDeleteChildren = 2124,

    // Database State Server
    DBSSObjectActivateWithDefaults = 2200,
    DBSSObjectActivateWithDefaultsOther = 2201,
    DBSSObjectGetActivated = 2207,
    DBSSObjectGetActivatedResp = 2208,
    DBSSObjectDeleteFieldDisk = 2230,
    DBSSObjectDeleteFieldsDisk = 2231,
    DBSSObjectDeleteDisk = 2232,

    // Database Server
    DBCreateObject = 3000,
    DBCreateObjectResp = 3001,
    DBObjectGetField = 3010,
    DBObjectGetFieldResp = 3011,
    DBObjectGetFields = 3012,
    DBObjectGetFieldsResp = 3013,
    DBObjectGetAll = 3014,
    DBObjectGetAllResp = 3015,
    DBObjectSetField = 3020,
    DBObjectSetFields = 3021,
    DBObjectSetFieldIfEquals = 3022,
    DBObjectSetFieldIfEqualsResp = 3023,
    DBObjectSetFieldsIfEquals = 3024,
    DBObjectSetFieldsIfEqualsResp = 3025,
    DBObjectSetFieldIfEmpty = 3026,
    DBObjectSetFieldIfEmptyResp = 3027,
    DBObjectDeleteField = 3030,
    DBObjectDeleteFields = 3031,
    DBObjectDelete = 3032,

    // Message Director (Control)
    MDAddChannel = 9000,
    MDRemoveChannel = 9001,
    MDAddRange = 9002,
    MDRemoveRange = 9003,
    MDAddPostRemove = 9010,
    MDClearPostRemoves = 9011,
}
