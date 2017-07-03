pipeline {
  agent any
  stages {
    stage('first') {
      steps {
        parallel(
          "first": {
            echo 'hui'
            
          },
          "": {
            echo 'world'
            
          }
        )
      }
    }
    stage('') {
      steps {
        echo 'hello'
      }
    }
  }
}