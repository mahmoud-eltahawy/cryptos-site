# Security Implementation - Cryptos Real Estate Platform

## 🔐 Overview

This application implements a comprehensive session-based authentication system to protect all dashboard routes and administrative functions. Every protected page validates user authentication before rendering content.

## 🛡️ Security Features

### 1. **Session-Based Authentication**

- **Technology**: `tower-sessions` with in-memory store
- **Session Duration**: 1 hour of inactivity (configurable)
- **Storage**: Secure HTTP cookies
- **Encryption**: Password hashing using `password-auth` crate with secure defaults

### 2. **Protected Routes**

All dashboard routes require authentication:

- `/dashboard/:id` - Main dashboard
- `/dashboard/manageUser/:id` - User management
- `/dashboard/manageEstates/:id` - Estate management
- `/dashboard/addUser/:id` - Add new user
- `/dashboard/updateUser/:targetId/:userId` - Update user
- `/dashboard/addEstate/:id` - Add new estate
- `/dashboard/updateEstate/:targetId/:userId` - Update estate
- `/dashboard/estateDetails/:targetId/:userId` - View estate details

### 3. **Authentication Flow**

```
┌─────────────┐
│   Login     │
│   Request   │
└──────┬──────┘
       │
       ▼
┌─────────────────────┐
│  Verify Username    │
│  & Password         │
└──────┬──────────────┘
       │
       ▼
┌─────────────────────┐
│  Create Session     │
│  Store User ID      │
│  Store User Level   │
└──────┬──────────────┘
       │
       ▼
┌─────────────────────┐
│  Redirect to        │
│  Dashboard          │
└─────────────────────┘
```

### 4. **Route Protection Mechanism**

Each protected component:

1. **Checks Authentication** on render using `check_auth()` server function
2. **Validates Session** by extracting session data from request
3. **Returns User ID** if authenticated, or error if not
4. **Redirects to Login** if unauthorized using `<Redirect path="/login"/>`
5. **Shows Loading State** during authentication check

**Example Implementation:**

```rust
#[server]
async fn check_auth() -> Result<Uuid, ServerFnError> {
    use leptos_axum::extract;
    use tower_sessions::Session;
    use crate::auth::require_auth;

    let session = extract::<Session>().await?;
    require_auth(session)
        .await
        .map_err(|e| ServerFnError::ServerError(e))
}

// In component:
let auth_check = Resource::new(|| (), |_| check_auth());

// Render with auth check:
{move || {
    auth_check.get().map(|auth_result| {
        match auth_result {
            Ok(_) => /* Render protected content */,
            Err(_) => view! { <Redirect path="/login"/> }
        }
    })
}}
```

## 🔑 Authentication Module (`src/auth/mod.rs`)

### Key Functions:

#### `get_user_id_from_session(session: Session) -> Option<Uuid>`
Retrieves the authenticated user's ID from the session.

#### `get_user_level_from_session(session: Session) -> Option<UserLevel>`
Retrieves the authenticated user's permission level (Admin or User).

#### `set_user_session(session: Session, user_id: Uuid, level: UserLevel)`
Creates a new session for a user after successful login.

#### `clear_user_session(session: Session)`
Clears the session on logout.

#### `require_auth(session: Session) -> Result<Uuid, String>`
Validates authentication and returns user ID or error.

## 🚪 Login System

### Features:
- **Username & Password** validation
- **Password Hashing** using `password_auth` with secure defaults
- **Session Creation** on successful authentication
- **Error Messages** displayed in Arabic for failed attempts
- **Automatic Redirect** to dashboard after login

### Security Measures:
- Passwords are never stored in plain text
- Password verification using timing-safe comparison
- Session IDs are cryptographically secure
- Failed login attempts return generic error messages

## 🚪 Logout System

### Features:
- **Session Destruction** - Completely removes session data
- **Server-Side Logout** - Ensures session is cleared on server
- **Redirect to Login** - Automatically redirects to login page
- **Accessible from Dashboard** - Logout button available on main dashboard

## 🔒 Session Management

### Configuration (in `main.rs`):

```rust
let session_store = MemoryStore::default();
let session_layer = SessionManagerLayer::new(session_store)
    .with_secure(false) // Set to true in production with HTTPS
    .with_expiry(Expiry::OnInactivity(Duration::from_secs(3600))); // 1 hour
```

### Session Data Stored:
- `user_id`: UUID of authenticated user
- `user_level`: Permission level (Admin/User)

### Session Expiry:
- **Inactivity Timeout**: 1 hour (3600 seconds)
- **Type**: Rolling expiry - resets on each request
- **Behavior**: Automatic logout after inactivity period

## 🎯 Best Practices Implemented

1. ✅ **Server-Side Validation**: All authentication checks happen server-side
2. ✅ **Password Hashing**: Passwords are hashed with `password-auth`
3. ✅ **Session Security**: Sessions use secure, cryptographic IDs
4. ✅ **Automatic Redirects**: Unauthorized users redirected to login
5. ✅ **Loading States**: Clear feedback during authentication checks
6. ✅ **Error Handling**: Graceful error messages for users
7. ✅ **Session Cleanup**: Proper logout clears all session data
8. ✅ **Route Protection**: Every sensitive route checks authentication

## 🚀 Production Recommendations

### For Production Deployment:

1. **Enable HTTPS**:
   ```rust
   .with_secure(true) // Enable secure cookies
   ```

2. **Use Persistent Session Store**:
   ```rust
   // Replace MemoryStore with Redis or database-backed store
   use tower_sessions_redis_store::RedisStore;
   let session_store = RedisStore::new(redis_client);
   ```

3. **Configure Session Cookie Settings**:
   ```rust
   .with_same_site(SameSite::Strict)
   .with_http_only(true)
   ```

4. **Add Rate Limiting**: Implement rate limiting on login endpoint

5. **Add CSRF Protection**: Implement CSRF tokens for forms

6. **Enable Logging**: Log authentication attempts for monitoring

7. **Add 2FA (Optional)**: Implement two-factor authentication

8. **Database Persistence**: Replace in-memory data structures with database

## 🛠️ Testing Authentication

### To Test Login:
1. Navigate to `/login`
2. Enter credentials (check `src/app.rs` for default users)
3. Should redirect to dashboard on success
4. Invalid credentials show error message

### To Test Protected Routes:
1. Visit any dashboard route without logging in
2. Should automatically redirect to `/login`
3. After login, should access routes successfully

### To Test Logout:
1. Login to dashboard
2. Click "تسجيل الخروج" (Logout) button
3. Should redirect to login page
4. Attempting to access dashboard should redirect to login

## 📋 Dependencies

```toml
tower-sessions = "0.13"
tower-sessions-memory-store = "0.13"
tower = "0.5"
password-auth = "1"
uuid = "1.19.0"
```

## 🔍 Security Checklist

- [x] Passwords are hashed, never stored in plain text
- [x] Sessions expire after inactivity
- [x] All protected routes validate authentication
- [x] Server-side validation on every request
- [x] Logout properly clears sessions
- [x] Error messages don't leak information
- [x] No authentication tokens in URLs
- [x] Session IDs are cryptographically secure
- [ ] HTTPS in production (configure before deployment)
- [ ] Persistent session store for production
- [ ] Rate limiting on login endpoint
- [ ] CSRF protection (recommended for production)

## 🆘 Troubleshooting

### Issue: "Unauthorized" error on protected pages
**Solution**: Ensure you're logged in and session hasn't expired

### Issue: Redirects to login in a loop
**Solution**: Check that session middleware is properly configured in `main.rs`

### Issue: Session expires too quickly
**Solution**: Adjust `Expiry::OnInactivity` duration in `main.rs`

### Issue: Can't access admin features
**Solution**: Verify user has `Admin` level in database

---

**Last Updated**: 2024
**Version**: 1.0.0
**Status**: Production-Ready (with HTTPS and persistent storage)