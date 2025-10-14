let { task, desc } = require("jake");

desc("This is the hello task.");
task("hello", function () {
    console.log("hello world!");
});

desc("This is the default task.");
task("default", function () {
    console.log("This is the default task.");
    console.log("Jake will run this task if you run `jake` with no task specified.");
});

desc("This is some other task. It depends on the default task");
task("otherTask", ["default"], function () {
    console.log("Some other task");
});
