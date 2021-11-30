function showErrorMessage(messagetext) {
    if (!document.getElementById("ErrorMessage")) {
      var div = document.createElement('div');
      div.setAttribute('class', 'error-msg');
      div.setAttribute('role', 'alert');
      div.setAttribute('id', 'ErrorMessage');
      document.body.appendChild(div);
    }
    document.getElementById("ErrorMessage").innerHTML = messagetext;
    document.getElementById("ErrorMessage").style.visibility = "visible";
  }
  
  function hideErrorMessage() {
    document.getElementById("ErrorMessage").style.visibility = "hidden";
  }
  
  function showErrorMessageWithTimer(messagetext, seconds = 0) {
    showErrorMessage(messagetext);
    if (seconds != 0) {
      var timer = window.setTimeout(hideErrorMessage, (seconds * 1000));
    }
  }
  
  function showSuccessMessage(messagetext) {
    if (!document.getElementById("SuccessMessage")) {
      var div = document.createElement('div');
      div.setAttribute('class', 'success-msg');
      div.setAttribute('role', 'alert');
      div.setAttribute('id', 'SuccessMessage');
      document.body.appendChild(div);
    }
    document.getElementById("SuccessMessage").innerHTML = messagetext;
    document.getElementById("SuccessMessage").style.visibility = "visible";
  }
  
  function hideSuccessMessage() {
    document.getElementById("SuccessMessage").style.visibility = "hidden";
  }
  
  function showSuccessMessageWithTimer(messagetext, seconds = 0) {
    showSuccessMessage(messagetext);
    if (seconds != 0) {
      var timer = window.setTimeout(hideSuccessMessage, (seconds * 1000));
    }
  }
  
  function showNotifyMessage(messagetext) {
    if (!document.getElementById("NotifyMessage")) {
      var div = document.createElement('div');
      div.setAttribute('class', 'notify-msg');
      div.setAttribute('id', 'NotifyMessage');
      document.body.appendChild(div);
    }
    document.getElementById("NotifyMessage").innerHTML = messagetext;
    document.getElementById("NotifyMessage").style.display = "inline-block";
    document.getElementById("NotifyMessage").style.visibility = "visible";
  }
  
  function startProgressSpinner() {
    if (!document.getElementById("ProgressSpinner")) {
      var div = document.createElement('div');
      div.setAttribute('class', 'progress-spinner');
      div.setAttribute('id', 'ProgressSpinner');
      document.body.appendChild(div);
    }
    document.getElementById("ProgressSpinner").style.visibility = "visible";
  }
  
  function stopProgressSpinner() {
    document.getElementById("ProgressSpinner").style.visibility = "hidden";
  }
  
  
  function accessWebService(url, okcallback, errorcallback, method = "GET", formdata = null, seconds = 0, timeout = 0) {
    var xhttp = new XMLHttpRequest();
  
    xhttp.onreadystatechange = function () {
      if (this.readyState == 4 && this.status == 200) {
        // Typical action to be performed when the document is ready:
        stopProgressSpinner();
        if (this.responseText.includes("ERROR:")) {
          showErrorMessageWithTimer(this.responseText, seconds);
          if (typeof errorcallback == "function") {
            errorcallback();
          }
        }
        else {
          if (typeof okcallback == "function") {
            okcallback(this.responseText);
          }
        }
      }
    };
    xhttp.ontimeout = function () {
      showErrorMessageWithTimer("Timeout: can not load data!", seconds);
      stopProgressSpinner();
    };
    xhttp.onabort = function () {
      showErrorMessageWithTimer("Timeout: loading the data was interrupted!", seconds);
      stopProgressSpinner();
    };
    xhttp.onerror = function () {
      showErrorMessageWithTimer("Timeout: error while loading the data!", seconds);
      stopProgressSpinner();
    };
    startProgressSpinner();
    xhttp.open(method, url, true);
    xhttp.timeout = timeout;
    if (method === "POST") {
      xhttp.setRequestHeader("Content-Type", "application/x-www-form-urlencoded", seconds);
    }
    xhttp.send(formdata);
  }
  
  function queryWebService(url, okcallback, errorcallback, seconds = 0, timeout = 0) {
    accessWebService(url, okcallback, errorcallback, "GET", null, seconds, timeout)
  }
  
  function sendToWebService(url, okcallback, errorcallback, formdata, seconds = 0, timeout = 0) {
    accessWebService(url, okcallback, errorcallback, "POST", formdata, seconds, timeout)
  }
  