import { betterAuth } from 'better-auth'
import pg from 'pg'

const pool = new pg.Pool({
  connectionString: process.env.DATABASE_URL || 'postgresql://smiled:smiled@localhost:5433/smiled',
  options: '-c search_path=auth',
})

export const auth = betterAuth({
  database: pool,
  baseURL: process.env.BETTER_AUTH_URL || 'http://localhost:3000',
  secret: process.env.BETTER_AUTH_SECRET,
  trustedOrigins: [
    process.env.NUXT_PUBLIC_APP_URL || 'http://localhost:3000',
  ],
  emailAndPassword: {
    enabled: true,
    minPasswordLength: 8,
    sendResetPassword: async ({ user, url }) => {
      // TODO: integrate with email service
      console.warn(`[DEV] Reset password for ${user.email}: ${url}`)
    },
  },
  session: {
    expiresIn: 60 * 60 * 8, // 8 hours
    updateAge: 60 * 30, // 30 minutes
  },
  advanced: {
    useSecureCookies: process.env.NODE_ENV === 'production',
  },
})
