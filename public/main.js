async function init() {
  let selectedRbutton = "grey";
  const rButtons = Array(...document.getElementsByName("func"));
  rButtons.forEach((but) => {
    but.addEventListener("click", () => {
      if (but.checked) selectedRbutton = but.value;
    });
  });
  let rustApp = null;
  try {
    rustApp = await import("../pkg");
  } catch (err) {
    console.error(err);

    return;
  }
  console.log(rustApp);
  const input = document.getElementById("upload");
  const fileReader = new FileReader();

  fileReader.onloadend = () => {
    let base64 = fileReader.result.replace(
      /^data:image\/(png|jpeg|jpg);base64,/,
      ""
    );
    let img_data_url;
    switch (selectedRbutton) {
      case "grey":
        img_data_url = rustApp.greyscale(base64);
        break;
      case "blur":
        img_data_url = rustApp.blur(base64);
        break;
      default:
        img_data_url = rustApp.greyscale(base64);
        break;
    }
    document.getElementById("new-img").setAttribute("src", img_data_url);
  };
  input.addEventListener("change", () => {
    fileReader.readAsDataURL(input.files[0]);
  });
}

init();
