const express = require('express');

const { getTestStatus } = require('../controllers/testController');

const router = express.Router();

router.get('/test', getTestStatus);

module.exports = router;
