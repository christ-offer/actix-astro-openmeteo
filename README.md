# Actix / Astro Playground

This is a playground for the [Actix](https://actix.rs/) and [Astro](https://astro.build/) web frameworks.

This is only meant as a learning project/playground and is not meant to be used in production or at all really.

The Rust code is probably not pretty nor great in any regards, I'm barely a web-developer.

Also, full discloure - I definitely got some Copilot/GPT assistance on the sorting function...


## Tech Stack

### Actix

The backend is written in Rust using the Actix web framework. 
Currently it only servers a static website on `/` and a JSON response on `/api/openmeteo`.

### Astro

The frontend is written in Astro. It is a static website that contains a Vue Island Component.

The vue component is a simple weather app that fetches data from the /api/openmeteo endpoint.


## Running the project

### Frontend

For dev purposes with hot reloading run:

```bash
cd www && pnpm dev
```

For production build run:

```bash
cd www && pnpm build
```


### Backend

For the static site to work you need to run build it first.

Then run the Backend with:

```bash
cargo run
```


## Endpoints

### /api/openmeteo

Takes the query parameters:

* `city` - The city to get the weather for
* `start_date` - The start date of the weather data
* `end_date` - The end date of the weather data

First it hits the OSM API to get the coordinates of the city. Then it hits the OpenMeteo API to get the weather data.

Returns a JSON response with the historical weather data from [OpenMeteo](https://openmeteo.org/).

It also adds a `statistics` object to the response that contains the max, min and mean values for all the relevant fields.