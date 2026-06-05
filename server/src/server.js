const path = require('path');

require('dotenv').config({ path: path.resolve(__dirname, '../.env') });
require('dotenv').config({ path: path.resolve(__dirname, '../../.env') });

const app = require('./app');

const port = Number(process.env.PORT) || 3000;

app.listen(port, () => {
  console.log(`NoneWhite_Site API server is running on port ${port}`);
});
