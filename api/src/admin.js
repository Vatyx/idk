/**
 * @file Initialize and configure resource for Firebase.
 */

const admin = require('firebase-admin');

console.log('Initializing admin firebase for PROD environment');
admin.initializeApp();

module.exports = admin;
