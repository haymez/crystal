const AgeReducer = require('./AgeReducer');

const ageReducer = new AgeReducer(process.argv[2]);

ageReducer.getAgeCounts();
