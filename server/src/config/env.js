module.exports = {
  database: {
    host: process.env.DB_HOST || 'localhost',
    name: process.env.DB_NAME || 'nonewhite_site',
    password: process.env.DB_PASSWORD || 'nonewhite_password',
    port: Number(process.env.DB_PORT) || 3306,
    user: process.env.DB_USER || 'nonewhite_user',
  },
  port: Number(process.env.PORT) || 3000,
};
