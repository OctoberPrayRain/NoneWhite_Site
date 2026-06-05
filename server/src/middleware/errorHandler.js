const { sendError } = require('../utils/response');

function notFoundHandler(req, res) {
  return sendError(res, 'API endpoint not found', 404, 404);
}

function errorHandler(err, req, res, next) {
  if (res.headersSent) {
    return next(err);
  }

  const status = err.status || 500;
  const code = err.code || status;
  const message = err.message || 'Internal server error';

  return sendError(res, message, code, status);
}

module.exports = {
  errorHandler,
  notFoundHandler,
};
