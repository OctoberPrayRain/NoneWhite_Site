const { sendSuccess } = require('../utils/response');

function getTestStatus(req, res) {
  return sendSuccess(res, {
    service: 'NoneWhite_Site API',
    status: 'ok',
  }, 'Backend test API is running');
}

module.exports = {
  getTestStatus,
};
