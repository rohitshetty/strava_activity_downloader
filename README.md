# Strava Activity Downloader

## What does this solve?

I want to sync my Strava activities to my blog (Which is made with Hugo). I want a cli tool that can list the activities, download the latest/selected activity, and download the gpx file, and metadata.

Following the unix way of doing things, I wanted this to be a standalone CLI tool instead of being part of blog's scripts/ folder.

Pretty sure there are several different tools that does this job already - but I want to build one to learn rust. Rust has been on my radar for a while to learn - so why not jump headfirst into this?

## Setup

You need to get the `refresh_token` and `access_token` with scope `activities:read_all`.

Strava lets you create apps on their platform. This provides `client_secret`, `client_id`, `access_id` (with default `read` scope). Open (API docs)[https://developers.strava.com/docs/getting-started/] and read Part B on how to setup the strava app and get your `client_secret` and `client_id`. The default access_id is not enough for listing activities. We need to "Authenticate" using oAuth to get the new access_code with updated scope.
This is a two step process.

1. Making call to Strava to begin the oAuth dance to authenticate and get the `code`. We need the scope: `activity:read_permission`
2. Exchange the code with strava to get the new `access_code` along with `refresh_token` and other metadata.

Refer to (API docs)[https://developers.strava.com/docs/getting-started/] Part C to know the details on executing 1 & 2 above.
If you face any issue with step 1, make sure to check Authorization callback domain in strava api section, and make sure it is localhost (or whatever you pass in the step 1 url).

Here is how it looks for me:

```
Call: http://www.strava.com/oauth/authorize?client_id=<client_id>&response_type=code&redirect_uri=http://localhost/exchange_token&approval_prompt=force&scope=activity:read_permission

Returns: http://localhost/exchange_token?state=&code=<code>&scope=read,activity:read_all

Make: curl -X POST https://www.strava.com/oauth/token \
	-F client_id=<client_id> \
	-F client_secret=<client_secret> \
	-F code=<code> \
	-F grant_type=authorization_code

Returns: {"token_type":"Bearer","expires_at":1715829651,"expires_in":21600,"refresh_token":"xxxxxxxx","access_token":"xxxxxxxx","athlete":{"id":19297002,"username":"rohit_shetty","resource_state":2,"firstname":"Rohit","lastname":"Shetty","bio":"","city":"Bangalore","state":"KARNATAKA","country":null,"sex":"M","premium":false,"summit":false,"created_at":"2017-01-11T02:16:50Z","updated_at":"2024-04-09T17:23:02Z","badge_type_id":0,"weight":106.0,"profile_medium":"https://lh3.googleusercontent.com/a/ACg8ocI5kDDnbnUj2t01f_Id1PHwJdMAHoz6xa8wFQiABWE6VlCVcPFO=s96-c","profile":"https://lh3.googleusercontent.com/a/ACg8ocI5kDDnbnUj2t01f_Id1PHwJdMAHoz6xa8wFQiABWE6VlCVcPFO=s96-c","friend":null,"follower":null}}
```

## Building

`cargo build`

## Running

`cargo run <subcommands>`

Currently supported subcommands:

`list` - list all strava activities

Refer to [changelog](./CHANGELOG.md) for changelogs and to [learnlogs](./LEARNLOG.md) for chronological updates on what I learnt building this.
