function sendSuccess(res, data = null, message = 'success', code = 0) {
  return res.json({
    code,
    data,
    message,
  });
}

function sendError(res, message = 'error', code = 500, status = 500, data = null) {
  return res.status(status).json({
    code,
    data,
    message,
  });
}

module.exports = {
  sendError,
  sendSuccess,
};
