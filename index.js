const ASMEditor = import('./pkg/webcam_stream_wasm');
ASMEditor.then((mod) => {
    console.log("initiated...");
    console.log(mod);
    window.mod = mod;
})

if (navigator.mediaDevices) {
  console.log('getUserMedia supported.');

  var constraints = { video: true };
  var chunks = [];

  // navigator.mediaDevices.getUserMedia(constraints)
  //   .then(function (stream) {
  //     var canvas = document.getElementById("video-src");
  //     var track = stream.getVideoTracks()[0];
  //     let imageCapture = new ImageCapture(track);
  //     setInterval(function () {
  //       imageCapture.grabFrame()
  //         .then(function (bmpImage) {
  //           canvas.width = bmpImage.width;
  //           canvas.height = bmpImage.height;
  //           canvas.getContext("2d").drawImage(bmpImage, 0, 0);
  //         })
  //         .catch(function (err) {
  //           console.log(err);
  //         });
  //     }, 100);
  //   })
  //   .catch(function (err) {
  //     console.log('The following error occurred: ' + err);
  //   })
}