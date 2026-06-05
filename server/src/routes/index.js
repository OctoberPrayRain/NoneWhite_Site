const express = require('express');

const testRoutes = require('./testRoutes');

const router = express.Router();

router.use(testRoutes);

module.exports = router;
