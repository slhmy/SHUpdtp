# Users

## Create `POST /users`
### Body `Json`
- `account`!
- `password`!
- `mobile`
- `role`!
### Return `null`
200 means success
### Explain
Method which enables you to create a new user.

*Currently mobile login is not supported. And server won't reject bad mobile strings.

## Login `POST /users/login`
### Body `Json`
- `account`!
- `password`!
### Return `Json`
SlimUser
- id
- role

## Check Oline Info `GET /users/{id}`
### Return `Json`
OutUser
- id
- account
- mobile
- role

## Logout `POST /users/logout`
### Return `null`
200 means success