# Goodest Doggo

Goodest doggo is a dog rating application the likes of which has never been seen
before. Backed by Vitess, with core application logic written in Rust, your dog
rating application will be experience better performance and availability than
most of the world's most popular applications. 

What really is goodest doggo? For you, it might be an opportunity to learn more
about Vitess, or maybe just rate some damn good looking dogs. Maybe you'll
decide to become a contributor and turn this into a dog dating app. Or use it as
a tool to begin a bourgeois revolution! The possibilities are limitless when you
have this many cute dogs at your fingertips!

## Setup

So now you are probably wondering, "How in the heck do I run this darn thing?"

Well, sit back, pour yourself a gin and tonic, and relax, because we are about
to setup the best damn dog rating app you've ever laid eyes on.

### Step 1

If you are a PS employee you can safely skip to
step 2. If you don't have access to the planetscale registry and you're not a
planetscale employee, then you should not have access to the planetscale
registry. Instead, you'll want to modify the `Makefile` so the `IMAGE` var is
set to your own custom registry. To build the newest docker image type `make
build`. Then to push it up to your registry just type `make push`.

### Step 2

Let's deploy that bad boy. What you'll want to do is modify the example file in
`deploy/goodestdoggo.yaml` to fit your use case. In the example it pulls the web
servers docker container from the planetscale registry, and sets a
`DATABASE_URL` environment variable. You will need to change the value for this
to the connection string of your vitess database where your puppers will live.
Let's move on now to setting up the Vitess database for this project.

## Vitess Setup

For this project we're going to setup the backing database using [CNDb](https://console.planetscale.com/). Create a new database in CNDb and call it `puppers`. Apply the schema found in `schema/puppers.sql` and after that apply the vschema ("sharding schema" in CNDb) in `schema/puppers.json`. We just create the puppers, ratings and users tables, and applied a sharding scheme for all three tables. Next we need to build sequence table that will generate unique ids for our pupper and rating tables. Within the same cluster create a new unsharded database called `pupper_sequences`. Apply the schema from `schema/lookup.sql` and the vschema from `schema/lookup.json` to this new CNDb unsharded database.

Now that we have applied our schemas and sharding schemas, we can begin
populating our database with the bestest doggos ever found!.

### Populating Puppers DB

To populate the puppers database first download the [Stanford dog training
set](http://vision.stanford.edu/aditya86/ImageNetDogs/). Now, setup a google
cloud bucket and run this script within a folder of the dog training set where
dog images live (inside one of the breed folders):

```sh
#!/bin/bash

BUCKET=bucketname
MYSQL=127.0.0.1

for image in `find . -name '*.jpg'`; do
    gsutil stat gs://$BUCKET/${image##*/} || ( gsutil cp $image gs://$BUCKET && gsutil acl ch -u AllUsers:R gs://$BUCKET/${image##*/} )
    NAME=$(xmllint --html --xpath "//div[@id='random_word']/text()" <(curl https://randomword.com 2>/dev/null))    
    echo "INSERT INTO puppers (name,image) VALUES (\"$NAME\",\"https://storage.googleapis.com/$BUCKET/${image##*/}\")" | mysql -h $MYSQL -u vtgate-user --port 3306 --password=SUPERSECRETPASSWORDHERE
    echo $image=$NAME    
done
```

This will load an image into your bucket, and then inject that dogs image url
along with a randomized name into the puppers database. Once you are satisfied
taht you've gotten enough dogs from a certain breed, change folders to a new
breed and run it again. Rinse and repeat until you feel you have a nice diverse
pool of amazing doggos loaded in.

## Enjoyment

At this stage you should have a distributed CNDb doggo database deployed, filled
with some amazing dogs, and your doge coin money has started rolling in. You are
surrounded by dozens of insanely cute dogs, drinking the gin and tonic you made
earlier, and laughing at how easy all of this was.

Enjoy!
