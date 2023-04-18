# The Robot Framework Tests

This training contains tests for each exercise.
To test your programm with those tests you must have already installed the prerequisites.
Refer to the documents in the Docs folder for the installation of Renode and Robot Framework.

After the set up is done, open a terminal and go in the folder of the exercise yo want to test, for example 1_Tests_GPIO for the exercise 1.
Execute the command :

```
renode-test ./QuestionYouWantToTest.robot
```

and the test will execute.

Robot Framework create multiple files in the current folder so you can have information on the test, that can be useful if the test didn't passed.
I recommend to always go in the folder of the exercise you want to test so you don't create information files in every folder and keep your repo clean.
