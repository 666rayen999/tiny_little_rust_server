# My Little AXUM Server
Just for testing...

### Features
- Blanzignly FAST
- 3.0mb memory
- 0.0% cpu usage
- create users
- create posts

### Usage
- run the ```srvr``` if u using linux (im using arch btw)
- or build+run it by running this command ```cargo r -r```
Then
```sh
# to see all users and all posts
curl localhost:3000/

# create user
curl localhost:3000/create/user_1
curl localhost:3000/create/user_2
# output:
#     0- user_1: []
#     1- user_2: []

# create post post
curl localhost:3000/user/0/create/post_1_by_0
# output: "post_1_by_0" by user_1
curl localhost:3000/user/0/create/post_2_by_0
# output: "post_2_by_0" by user_1
curl localhost:3000/user/1/create/post_2_by_1
# output: "post_2_by_1" by user_2

# show user by id
curl localhost:3000/user/0
# output:
#     user_1 = ["post_1_by_0", "post_2_by_0"]

# show all users
curl localhost:3000/users
# show all posts
curl localhost:3000/posts
```

# License
UNDER MIT.
