# api v1

To call the api, POST on /api/v1/{endpoint}

## Table of Contents

- [Request Format](#request-format)
- [Response Format](#response-format)
- [Authentication](#authentication)
- [Semantics](#semantics)
- [Endpoints](#endpoints)
  - [auth](#auth)
  - [users](#users)
  - [cards](#cards)

## Request Format

```json
{
    "type": "create_stack", // Required
    "content": { // Optional
        "unique_id": "uzn1lKkFF00",         // Optional
        "email": "john.smith@hotmail.com",  // Optional
        "password": "Dupa123!",             // Optional
        "email": "john.smith@hotmail.com",  // Optional
        "username": "FlashCardEnjoyer69",   // Optional
        "password": "Dupa123!",             // Optional
        "country": "GBR",                   // Optional
        "name": "My first stack",           // Optional
        "tags": "favourites,my stacks",     // Optional
        "visibility": true,                 // Optional
        "stack_id": "vbCQQB1M_nE",          // Optional
        "frontside": "Question",            // Optional
        "backside": "Answer"                // Optional
    }
}
```

## response Format

```json
{
    "status": "ok", // Guaranteed
    "content": { // Optional
        "errors": [
            { // Optional
                "code": 410,                    // Guaranteed
                "message": "Invalid content"    // Guaranteed
            }
        ],
        "user": { // Optional
            "unique_id": "H8ZIe_honK",          // Guaranteed
            "email": "john.smith@hotmail.com",  // Optional
            "username": "FlashCardEnjoyer69",   // Guaranteed
            "date_of_registration": 1697855501, // Guaranteed
            "country": "GBR"                    // Guaranteed
        },
        "stacks": [ // Optional
            { // Optional
                "unique_id": "vbCQQB1M_nE",     // Guaranteed
                "owner_id": "H8ZIe_honK",       // Guaranteed
                "name": "My first stack",       // Guaranteed
                "visibility": true,             // Guaranteed
                "cards_count": 2,               // Guaranteed
                "tags": "favourites,my stacks", // Guaranteed
            },
        ],
        "cards": [ // Optional
            { // Optional
                "unique_id": "uzn1lKkFF00", // Guaranteed
                "stack_id": "vbCQQB1M_nE",  // Guaranteed
                "frontside": "Question",    // Guaranteed
                "backside": "Answer"        // Guaranteed
            },
        ],
    },
}
```

## Authentication

authentication will be handled with cookies. Since i want to get the API ASAP i won't do much except of basic token authntication.  
A valid authentication is required only when accessing private data, or performing action.  
When doing test in browsers everything should happen authomatically, but for test you're gonna need to include jwt_v1 cookie in your headers.  
Features like refresh tokens and token blacklisting are on my to do list.

## Semantics

- unique_id - A unique id assigne for each user, stack, and card.  
- password requirements:
  - 7 characters long
  - 1 small letter
  - 1 capital letter
  - 1 special character
- username has to be made with ASCII character only and be between 5 and 25 characters long
- API will respond with http 200 for valid requests and 400 for invalid requests.
- If the request is valid but no data is found it will return an empty object or array, like so:
  ```json
  {
    "status":"ok",
    "content": {
        "cards": []
    }
  }
  ```



<details>
  <summary>List of valid country codes</summary>
<br/>
AFG - Afghanistan<br/>
ALB - Albania<br/>
DZA - Algeria<br/>
ASM - American Samoa<br/>
AND - Andorra<br/>
AGO - Angola<br/>
AIA - Anguilla<br/>
ATA - Antarctica<br/>
ATG - Antigua and Barbuda<br/>
ARG - Argentina<br/>
ARM - Armenia<br/>
ABW - Aruba<br/>
AUS - Australia<br/>
AUT - Austria<br/>
AZE - Azerbaijan<br/>
BHS - Bahamas<br/>
BHR - Bahrain<br/>
BGD - Bangladesh<br/>
BRB - Barbados<br/>
BLR - Belarus<br/>
BEL - Belgium<br/>
BLZ - Belize<br/>
BEN - Benin<br/>
BMU - Bermuda<br/>
BTN - Bhutan<br/>
BOL - Bolivia<br/>
BES - Bonaire, Sint Eustatius and Saba<br/>
BIH - Bosnia and Herzegovina<br/>
BWA - Botswana<br/>
BVT - Bouvet Island<br/>
BRA - Brazil<br/>
IOT - British Indian Ocean Territory<br/>
BRN - Brunei Darussalam<br/>
BGR - Bulgaria<br/>
BFA - Burkina Faso<br/>
BDI - Burundi<br/>
KHM - Cambodia<br/>
CMR - Cameroon<br/>
CAN - Canada<br/>
CPV - Cape Verde<br/>
CYM - Cayman Islands<br/>
CAF - Central African Republic<br/>
TCD - Chad<br/>
CHL - Chile<br/>
CHN - China<br/>
CXR - Christmas Island<br/>
CCK - Cocos (Keeling) Islands<br/>
COL - Colombia<br/>
COM - Comoros<br/>
COG - Congo (Congo-Brazzaville)<br/>
COD - Democratic Republic of the Congo (Congo-Kinshasa)<br/>
COK - Cook Islands<br/>
CRI - Costa Rica<br/>
HRV - Croatia<br/>
CUB - Cuba<br/>
CUW - Curaçao<br/>
CYP - Cyprus<br/>
CZE - Czechia<br/>
DNK - Denmark<br/>
DJI - Djibouti<br/>
DMA - Dominica<br/>
DOM - Dominican Republic<br/>
TLS - East Timor<br/>
ECU - Ecuador<br/>
EGY - Egypt<br/>
SLV - El Salvador<br/>
GNQ - Equatorial Guinea<br/>
ERI - Eritrea<br/>
EST - Estonia<br/>
SWZ - Eswatini (fmr. "Swaziland")<br/>
ETH - Ethiopia<br/>
FLK - Falkland Islands<br/>
FRO - Faroe Islands<br/>
FJI - Fiji<br/>
FIN - Finland<br/>
FRA - France<br/>
GUF - French Guiana<br/>
PYF - French Polynesia<br/>
ATF - French Southern Territories<br/>
GAB - Gabon<br/>
GMB - The Gambia<br/>
GEO - Georgia<br/>
DEU - Germany<br/>
GHA - Ghana<br/>
GIB - Gibraltar<br/>
GRC - Greece<br/>
GRL - Greenland<br/>
GRD - Grenada<br/>
GLP - Guadeloupe<br/>
GUM - Guam<br/>
GTM - Guatemala<br/>
GGY - Guernsey<br/>
GIN - Guinea<br/>
GNB - Guinea-Bissau<br/>
GUY - Guyana<br/>
HTI - Haiti<br/>
HMD - Heard Island and McDonald Islands<br/>
VAT - Holy See (Vatican City State)<br/>
HND - Honduras<br/>
HKG - Hong Kong<br/>
HUN - Hungary<br/>
ISL - Iceland<br/>
IND - India<br/>
IDN - Indonesia<br/>
IRN - Iran<br/>
IRQ - Iraq<br/>
IRL - Ireland<br/>
IMN - Isle of Man<br/>
ISR - Israel<br/>
ITA - Italy<br/>
CIV - Ivory Coast (Côte d'Ivoire)<br/>
JAM - Jamaica<br/>
JPN - Japan<br/>
JEY - Jersey<br/>
JOR - Jordan<br/>
KAZ - Kazakhstan<br/>
KEN - Kenya<br/>
KIR - Kiribati<br/>
PRK - North Korea<br/>
KOR - South Korea<br/>
KWT - Kuwait<br/>
KGZ - Kyrgyzstan<br/>
LAO - Laos<br/>
LVA - Latvia<br/>
LBN - Lebanon<br/>
LSO - Lesotho<br/>
LBR - Liberia<br/>
LBY - Libya<br/>
LIE - Liechtenstein<br/>
LTU - Lithuania<br/>
LUX - Luxembourg<br/>
MAC - Macau<br/>
MDG - Madagascar<br/>
MWI - Malawi<br/>
MYS - Malaysia<br/>
MDV - Maldives<br/>
MLI - Mali<br/>
MLT - Malta<br/>
MHL - Marshall Islands<br/>
MTQ - Martinique<br/>
MRT - Mauritania<br/>
MUS - Mauritius<br/>
MYT - Mayotte<br/>
MEX - Mexico<br/>
FSM - Federated States of Micronesia<br/>
MDA - Moldova<br/>
MCO - Monaco<br/>
MNG - Mongolia<br/>
MNE - Montenegro<br/>
MSR - Montserrat<br/>
MAR - Morocco<br/>
MOZ - Mozambique<br/>
MMR - Myanmar (formerly Burma)<br/>
NAM - Namibia<br/>
NRU - Nauru<br/>
NPL - Nepal<br/>
NLD - Netherlands<br/>
NCL - New Caledonia<br/>
NZL - New Zealand<br/>
NIC - Nicaragua<br/>
NER - Niger<br/>
NGA - Nigeria<br/>
NIU - Niue<br/>
NFK - Norfolk Island<br/>
MNP - Northern Mariana Islands<br/>
NOR - Norway<br/>
OMN - Oman<br/>
PAK - Pakistan<br/>
PLW - Palau<br/>
PSE - State of Palestine<br/>
PAN - Panama<br/>
PNG - Papua New Guinea<br/>
PRY - Paraguay<br/>
PER - Peru<br/>
PHL - Philippines<br/>
PCN - Pitcairn Islands<br/>
PMR - Pridnestrovian Moldavian Republic<br/>
POL - Poland<br/>
PRT - Portugal<br/>
PRI - Puerto Rico<br/>
QAT - Qatar<br/>
MKD - North Macedonia<br/>
ROU - Romania<br/>
RUS - Russia<br/>
RWA - Rwanda<br/>
REU - Réunion<br/>
BLM - Saint Barthélemy<br/>
SHN - Saint Helena, Ascension, and Tristan da Cunha<br/>
KNA - Saint Kitts and Nevis<br/>
LCA - Saint Lucia<br/>
MAF - Saint Martin (French part)<br/>
SPM - Saint Pierre and Miquelon<br/>
VCT - Saint Vincent and the Grenadines<br/>
WSM - Samoa<br/>
SMR - San Marino<br/>
STP - São Tomé and Príncipe<br/>
SAU - Saudi Arabia<br/>
SEN - Senegal<br/>
SRB - Serbia<br/>
SYC - Seychelles<br/>
SLE - Sierra Leone<br/>
SGP - Singapore<br/>
SXM - Sint Maarten (Dutch part)<br/>
SVK - Slovakia<br/>
SVN - Slovenia<br/>
SLB - Solomon Islands<br/>
SOM - Somalia<br/>
ZAF - South Africa<br/>
SGS - South Georgia and the South Sandwich Islands<br/>
SSD - South Sudan<br/>
ESP - Spain<br/>
LKA - Sri Lanka<br/>
SDN - Sudan<br/>
SUR - Suriname<br/>
SJM - Svalbard and Jan Mayen<br/>
SWE - Sweden<br/>
CHE - Switzerland<br/>
SYR - Syria<br/>
TWN - Taiwan<br/>
TJK - Tajikistan<br/>
TZA - Tanzania<br/>
THA - Thailand<br/>
TGO - Togo<br/>
TKL - Tokelau<br/>
TON - Tonga<br/>
TTO - Trinidad and Tobago<br/>
TUN - Tunisia<br/>
TUR - Turkey<br/>
TKM - Turkmenistan<br/>
TCA - Turks and Caicos Islands<br/>
TUV - Tuvalu<br/>
UGA - Uganda<br/>
UKR - Ukraine<br/>
ARE - United Arab Emirates<br/>
GBR - United Kingdom<br/>
USA - United States<br/>
UMI - United States Minor Outlying Islands<br/>
URY - Uruguay<br/>
UZB - Uzbekistan<br/>
VUT - Vanuatu<br/>
VEN - Venezuela<br/>
VNM - Vietnam<br/>
VIR - United States Virgin Islands<br/>
WLF - Wallis and Futuna<br/>
ESH - Western Sahara<br/>
YEM - Yemen<br/>
ZMB - Zambia<br/>
ZWE - Zimbabwe<br/>
</details>

## Endpoints

### auth

Post to this endpoint to create an authentication token.  
Only when returns the ok status is the authentication cookie set.

#### Request Format
##### Types:
- authenticate
- check
- logout
##### Content:
- email
- password

Examples:
```json
{
    "type": "authenticate",
    "content": {
        "email": "john.smith@hotmail.com",
        "password": "Dupa123!"
    }
}
```
```json
{
    "type": "check",
}
```
```json
{
    "type": "logout",
}
```

#### Response Format
##### Content:
- errors
- unique_id

```json
{
    "status": "ok"
}
```
```json
{
    "status": "ok",
    "content": {
        "unique_id": "dp1OnbRDeP"
    }
}
```
```json
{
    "status":"err",
    "content": {
        "errors": [
            {
                "code": 410,
                "message": "Invalid content"
            }
        ]
    }
}
```
```json
{
    "status":"err",
    "content": {
        "errors": [
            {
                "code": 411,
                "message": "Invalid email or password"
            }
        ]
    }
}
```

### users

Post to this endpoint to access and menage user data.  
**get_user** is used for accessing public data and **get_my_profile** will get private information as well, but requires authentication.

#### Request Format
##### Types:
- get_my_profile
- get_user
- create_user
- update_user
- delete_user
##### Content:
- errors
- user
  - email
  - username
  - password
  - date_of_registration
  - country

Examples:
```json
{
    "type": "get_user",
    "content": {
        "unique_id": "H8ZIe_honK"
    }
}
```
```json
{
    "type": "get_user",
    "content": {
        "username": "FlashCardEnjoyer69"
    }
}
```
```json
{
    "type": "create_user",
    "content": {
        "email": "john.smith@hotmail.com",
        "username": "FlashCardEnjoyer69",
        "password": "Dupa123!",
        "country": "GBR"
    }
}
```
All present content keys will be modified. The user that is being modified will be fetched from the cookie authentication.
```json
{
    "type": "update_user",
    "content": {
        "email": "john.smith@hotmail.com",
        "username": "FlashCardEnjoyer69",
        "password": "Dupa123!",
        "country": "GBR"
    }
}
```
```json
{
    "type": "delete_user",
    "content": {
        "password": "Dupa123!",
    }
}
```

#### Response Format
##### Content:
- errors
- user

For get_user
```json
{
    "status":"ok",
    "content": {
        "user": {
            "unique_id": "H8ZIe_honK",
            "username": "FlashCardEnjoyer69",
            "date_of_registration": 1697855501,
            "country": "GBR"
        }
    }
}
```
```json
{
    "status":"ok"
}
```
```json
{
    "status":"err",
    "content": {
        "errors": [
            {
                "code": 411,
                "message": "Invalid email or password"
            }
        ]
    }
}
```

### cards

Post to this endpoint to access and menage cards and stacks data.  

#### Request Format
##### Types:
- get_stacks_by_owner_id
- get_stack_by_id
- get_cards_by_stack_id
- get_card_by_id
- create_stack
- create_card
- update_stack
- update_card
- delete_stack
- delete_card
##### Content:
- errors
- stacks (list of objects)
  - name
  - cards_count
  - tags
  - visibility
- cards (list of objects)
  - unique_id
  - frontside
  - backside

Examples:
```json
{
    "type": "get_stacks_by_owner_id",
    "content": {
        "unique_id": "H8ZIe_honK"
    }
}
```
```json
{
    "type": "get_stack_by_id",
    "content": {
        "unique_id": "H8ZIe_honK"
    }
}
```
```json
{
    "type": "create_stack",
    "content": {
        "name": "My first stack",
        "tags": "favourites,my stacks",
        "visibility": true
    }
}
```
All present content keys will be modified
```json
{
    "type": "update_card",
    "content": {
        "unique_id": "H8ZIe_honK",
        "tags": "favourites,my stacks"
    }
}
```
```json
{
    "type": "delete_card",
    "content": {
        "unique_id": "H8ZIe_honK"
    }
}
```

#### Response Format
##### Content:
- errors
- stacks
- cards

```json
{
    "status":"ok",
    "content": {
        "stacks": [
            {
                "unique_id": "vbCQQB1M_nE",
                "owner_id": "H8ZIe_honK",
                "name": "My first stack",
                "visibility": true,
                "cards_count": 2,
                "tags": "favourites,my stacks",
            },
            {
                "unique_id": "MFJLTUULcOs",
                "owner_id": "H8ZIe_honK",
                "name": "My seccond stack",
                "visibility": true,
                "cards_count": 0,
                "tags": "",
            }
        ]
    }
}
```
```json
{
    "status":"ok",
    "content": {
        "cards": [
            {
                "unique_id": "uzn1lKkFF00",
                "stack_id": "vbCQQB1M_nE",
                "frontside": "Question",
                "backside": "Answer"
            },
            {
                "unique_id": "FpSfTnQsG2Y",
                "stack_id": "vbCQQB1M_nE",
                "frontside": "Question 2",
                "backside": "Answer 2"
            }
        ]
    }
}
```
if no cards like requested are found.
```json
{
    "status":"ok",
    "content": {
        "cards": []
    }
}
```
```json
{
    "status":"ok"
}
```
```json
{
    "status":"err",
    "content": {
        "errors": [
            {
                "code": 410,
                "message": "Invalid content"
            }
        ]
    }
}
```
