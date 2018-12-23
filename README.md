# crier
Rust web service for handling QR-code based payments through Stripe. Heavy WIP one-coder hobbyist job, not production ready but may be interesting if you want to see how to put together a Rust web app or are some kind of recruiter I suppose.

Primarily a learning project. It has lots of dependencies (check cargo.toml) but most heavily depends on Iron (web/http framework), Maud (html templating with neat DSLs - look at any file with "render" in the name to see how this is used) and Diesel (an ORM).

## Requirements

To compile, you'll need to install libssl so that things can communicate over https.

You will need to install PostgreSQL, Redis and have a sandbox Stripe account in order to run this application. 

To set up the database, you'll need to install the Diesel command line tool and run diesel migration setup from the root directory of this repository. This will have Diesel connect to the database, run the database migrations and update the src/schema.rs file.

The following environment variables should be set, these can also be picked up from a .env file automatically:

DATABASE_URL - this should look something like postgres://user:password@127.0.0.1/crier

REDIS_URL - this should look something like redis://127.0.0.1 if you're using a out of the box config.

For the above URLs, please check the products' manuals for details on how to use other hostnames, ports and auth details.

STRIPE_CLIENT_ID, STRIPE_SECRET_KEY, STRIPE_PUBLISHABLE_KEY - you can retrieve these from your stripe dashboard

## Licensing
AGPLv3 - tldr; if you modify this code base and then host it anywhere, in any form, you have to release your source code modifications under the AGPLv3 license. For more information see LICENSE
