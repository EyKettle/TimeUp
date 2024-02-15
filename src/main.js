const invoke = window.__TAURI__.invoke;
const { appWindow } = window.__TAURI__.window;
const { notification } = window.__TAURI__.notification;

document
  .getElementById('titlebar-minimize')
  .addEventListener('click', function () {
    appWindow.minimize();
  })
document
  .getElementById('titlebar-close')
  .addEventListener('click', function () {
    appWindow.close();
  });

let audios = ['assets/audio/TaskFinish.wav', 'assets/audio/TimeFinish.wav'];
function audio_play(index) {
  var audio = document.createElement('audio');
  audio.src = audios[index];
  audio.play();
}

function input_lostFocus(event) {
  if (event.key == 'Enter' || event.key == 'Escape') {
    event.target.blur();
  }
}

function nameup(event) {
  if (event.key == 'Enter' && event.target.value != "") {
    event.target.disabled = true;
    if (event.target.parentElement.children[0].textContent == "1") {
      event.target.style.color = "#888";
    }
    event.target.blur();
    unnameTasks--;
    data_save();
  }
}

let listbox = document.getElementById('taskList');
let unnameTasks = 0;
document.addEventListener('keydown', async function (event) {
  if (page0.style.pointerEvents != 'none') {
    if (event.ctrlKey && event.key == 't') {
      data_canchange = false
      invoke('datatest_read').then(result => {
        if (result) {
          listbox.innerHTML = '';
          document.getElementById('Titlebar-Title').textContent = result[0];
          result[1].forEach(e => {
            task_add(e.description, e.completed == 'Completed');
            nameup(listbox.lastElementChild.children[1].dispatchEvent(new KeyboardEvent(
              'keydown', {
              key: 'Enter'
            }
            )));
          });
        }
        data_canchange = true
      })
    }
    if (event.ctrlKey && event.key == 'n') {
      event.preventDefault();
      task_add();
    }
    if (event.ctrlKey && event.key == 'ArrowUp') {
      event.preventDefault();
      if (unnameTasks <= 0) {
        return;
      }
      let tg;
      if (document.activeElement.tagName == 'INPUT' && document.activeElement.parentElement.parentElement == listbox) {
        tg = document.activeElement.parentElement;
      }
      else {
        tg = listbox.lastElementChild;
      }
      do {
        tg = listbox.children[(Array.from(listbox.children).indexOf(tg) - 1) < 0 ? listbox.childElementCount - 1 : Array.from(listbox.children).indexOf(tg) - 1];
      } while (tg.children[1].disabled);
      tg.children[1].focus();
    }
    if (event.ctrlKey && event.key == 'ArrowDown') {
      event.preventDefault();
      if (unnameTasks <= 0) {
        return;
      }
      let tg;
      if (document.activeElement.tagName == 'INPUT' && document.activeElement.parentElement.parentElement == listbox) {
        tg = document.activeElement.parentElement;
      }
      else {
        tg = listbox.firstElementChild;
      }
      do {
        tg = listbox.children[(Array.from(listbox.children).indexOf(tg) + 1) > (listbox.childElementCount - 1) ? 0 : Array.from(listbox.children).indexOf(tg) + 1];
      } while (tg.children[1].disabled);
      tg.children[1].focus();
    }
  }
  if (page1.style.pointerEvents != 'none') {
    if (/^[0-9]$/.test(event.key)) {
      if (clockMin.style.transform == 'translateY(-4px)') {
        clockMin.textContent = "00";
      }
    }
  }
}, true);
function task_add(taskname = "", isFinished) {
  let item = document.createElement('div');
  item.classList.add('list-item');
  let icon = document.createElement('label');
  icon.classList.add('icons');
  icon.style.marginRight = '8px';
  icon.style.pointerEvents = 'none';
  icon.textContent = "0";
  let task = document.createElement('input');
  task.classList.add('task-input');
  task.value = taskname;
  task.addEventListener('keydown', nameup);
  task.addEventListener('keydown', function (event) {
    if (event.key == 'Escape') {
      if (task.value == "") {
        menuDel.dispatchEvent(new MouseEvent('click', {
          bubbles: true,
          cancelable: true,
          view: window,
        }));
      }
      else {
        task.disabled = true;
        if (icon.textContent == "1") {
          task.style.color = "#888";
        }
        task.blur();
      }
    }
  })
  if (isFinished) {
    task.disabled = true;
    icon.textContent = "1";
    icon.style.color = "#888";
    task.style.color = "#888";
  }
  let menuBorder = document.createElement('div');
  menuBorder.style.display = 'inline-flex';
  menuBorder.style.overflow = 'hidden';
  menuBorder.style.borderRadius = '10px';
  menuBorder.style.opacity = '0';
  menuBorder.style.pointerEvents = 'none';
  menuBorder.style.position = 'absolute';
  menuBorder.style.insetInline = '2px';
  menuBorder.style.justifyContent = 'right';
  menuBorder.style.transform = 'scale(0.8)';
  menuBorder.style.transition = 'transform 0.25s cubic-bezier(0, 0, 0, 1), opacity 0.25s';
  menuBorder.addEventListener('contextmenu', function (event) {
    event.preventDefault();
    if (task.disabled) {
      item.children[0].style.opacity = '1';
      item.children[0].style.filter = 'blur(0)';
      item.children[1].style.opacity = '1';
      item.children[1].style.filter = 'blur(0)';
      menuBorder.style.opacity = '0';
      menuBorder.style.transform = 'scale(0.8)';
      menuBorder.style.pointerEvents = 'none';
    }
  })
  let menuDel = document.createElement('div');
  menuDel.textContent = "d";
  menuDel.classList.add('icons');
  menuDel.classList.add('button-flat');
  menuDel.addEventListener('click', async function () {
    item.pointerEvents = 'none';
    item.style.transform = 'scale(0.6)';
    item.style.filter = 'blur(8px)';
    item.style.opacity = '0';
    await new Promise(resolve => setTimeout(resolve, 250));
    unnameTasks--;
    item.remove();
  })
  menuBorder.appendChild(menuDel);
  let menuSet = document.createElement('div');
  menuSet.textContent = "s";
  menuSet.classList.add('icons');
  menuSet.classList.add('button-flat');
  menuSet.addEventListener('click', function () {
    menuBorder.dispatchEvent(new MouseEvent('contextmenu', {
      bubbles: true,
      cancelable: true,
      view: window,
    }));
    task.disabled = false;
    unnameTasks++;
    task.focus();
  })
  menuBorder.appendChild(menuSet);
  item.appendChild(icon);
  item.appendChild(task);
  item.appendChild(menuBorder);
  item.addEventListener('click', function (event) {
    if (event.target == item && icon.style.visibility != "hidden") {
      if (icon.textContent == "1") {
        icon.textContent = "0";
        icon.style.color = "";
        task.style.color = "";
      }
      else {
        icon.textContent = "1";
        icon.style.color = "#888";
        task.style.color = "#888";
        audio_play(0);
      }
      data_save();
    }
  });
  item.addEventListener('contextmenu', function (event) {
    if (event.target == item) {
      event.preventDefault();
      if (task.disabled) {
        event.target.children[0].style.opacity = '0';
        event.target.children[0].style.filter = 'blur(4px)';
        event.target.children[1].style.opacity = '0';
        event.target.children[1].style.filter = 'blur(4px)';
        event.target.lastElementChild.style.opacity = '1';
        event.target.lastElementChild.style.transform = 'scale(1)';
        event.target.lastElementChild.style.pointerEvents = 'unset';
      }
    }
  });
  listbox.appendChild(item);
  listbox.parentElement.scrollTop = listbox.scrollHeight;
  if (!isFinished) {
    listbox.lastElementChild.children[listbox.lastElementChild.children.length - 2].focus();
    unnameTasks++;
  }
  item.style.transform = 'scale(1)';
  item.style.filter = 'blur(0)';
  item.style.opacity = '1';
  data_save();
}

let page0 = document.getElementById('page0');
let page1 = document.getElementById('page1');
let page2 = document.getElementById('page2');
page0.addEventListener('wheel', function (event) {
  event.stopPropagation();
})
document.getElementById('taskAdd').addEventListener('click', function () {
  task_add();
});
let toolbarTimer = document.getElementById('pageTimer');
toolbarTimer.addEventListener('click', async function () {
  if (toolbarSetting.textContent == "R") {
    page1.style.opacity = '1';
    page1.style.filter = 'blur(0)';
    page1.style.transform = 'scale(1)';
    page1.style.pointerEvents = 'unset';
    page2.style.pointerEvents = 'none';
    page1.style.translate = '0 0';
    page2.style.translate = '100% 0'
    toolbarSetting.style.color = 'unset';
    toolbarSetting.textContent = "S"
    toolbarTimer.style.color = '#396cd8';
    toolbarTimer.textContent = "U"
  }
  else {
    if (toolbarTimer.textContent == "T") {
      toolbarTimer.style.color = '#396cd8';
      toolbarTimer.textContent = "U"
      page0.style.opacity = '0';
      page0.style.transform = 'scale(0.6)';
      page0.style.filter = 'blur(4px)';
      page0.style.pointerEvents = 'none';
      page1.style.opacity = '1';
      page1.style.transform = 'scale(1)';
      page1.style.filter = 'blur(0)';
      page1.style.pointerEvents = 'unset';
      page1.style.translate = '0 0';
      page2.style.translate = '100% 0'
    }
    else {
      toolbarTimer.style.color = 'unset';
      toolbarTimer.textContent = "T"
      page0.style.opacity = '1';
      page0.style.transform = 'scale(1)';
      page0.style.filter = 'blur(0)';
      page0.style.pointerEvents = 'unset';
      page1.style.opacity = '0';
      page1.style.transform = 'scale(1.4)';
      page1.style.filter = 'blur(4px)';
      page1.style.pointerEvents = 'none';
    }
  }
});
let toolbarSetting = document.getElementById('pageSetting');
toolbarSetting.addEventListener('click', async function () {
  if (toolbarTimer.textContent == "U") {
    page2.style.opacity = '1';
    page2.style.filter = 'blur(0)';
    page2.style.transform = 'scale(1)';
    page2.style.pointerEvents = 'unset';
    page1.style.pointerEvents = 'none';
    page1.style.translate = '-100% 0';
    page2.style.translate = '0 0';
    toolbarTimer.style.color = 'unset';
    toolbarTimer.textContent = "T"
    toolbarSetting.style.color = '#396cd8';
    toolbarSetting.textContent = "R"
  }
  else {
    if (toolbarSetting.textContent == "S") {
      toolbarSetting.style.color = '#396cd8';
      toolbarSetting.textContent = "R"
      page0.style.opacity = '0';
      page0.style.transform = 'scale(0.6)';
      page0.style.filter = 'blur(4px)';
      page0.style.pointerEvents = 'none';
      page2.style.opacity = '1';
      page2.style.transform = 'scale(1)';
      page2.style.filter = 'blur(0)';
      page2.style.pointerEvents = 'unset';
      page2.style.translate = '0 0';
      page1.style.translate = '-100% 0';
    }
    else {
      toolbarSetting.style.color = 'unset';
      toolbarSetting.textContent = "S"
      page0.style.opacity = '1';
      page0.style.transform = 'scale(1)';
      page0.style.filter = 'blur(0)';
      page0.style.pointerEvents = 'unset';
      page2.style.opacity = '0';
      page2.style.transform = 'scale(1.4)';
      page2.style.filter = 'blur(4px)';
      page2.style.pointerEvents = 'none';
    }
  }
});

let clockMin = document.getElementById('clock-min');
let clockS = document.getElementById('clock-s');
clockMin.addEventListener('keyup', input_lostFocus);
clockS.addEventListener('keyup', input_lostFocus);

document
  .getElementById('timerPlay')
  .addEventListener('click', function (event) {
    clockMin.disabled = !clockMin.disabled;
    clockS.disabled = !clockS.disabled;
    if (event.target.textContent == 'p') {
      event.target.textContent = 'u';
      timerStart();
    }
    else {
      event.target.textContent = 'p';
      timerStop();
    }
  })
let timerCounter;
function setClockValue(input) {
  let value = parseInt(input.value);
  input.value = input.value == '' ? '00' : value.toString().substr(value.toString().length - 2, 2);
  if (value > 59) {
    input.value = '59';
    return;
  }
  if (value.toString().length == 1) {
    input.value = '0' + value.toString();
  }
}
function timerStart() {
  let bef = clockMin.value + clockS.value;
  timerCounter = setInterval(function () {
    if (clockS.value == '00') {
      clockMin.value = parseInt(clockMin.value) - 1;
      clockS.value = '60';
    }
    clockS.value = parseInt(clockS.value) - 1;
    setClockValue(clockMin);
    setClockValue(clockS);
    if (clockMin.value == '00' && clockS.value == '00') {
      document.getElementById('timerPlay').textContent = 'p';
      clockMin.disabled = false;
      clockS.disabled = false;
      clearInterval(timerCounter);
      clockMin.value = bef.substring(0, 2);
      clockS.value = bef.substring(2, 4);
      notifiBack.style.opacity = '1'
      notifiContent.style.opacity = '1'
      notifiContent.style.clipPath = 'circle(100% at 50% 50%)';
      notifiBack.style.clipPath = 'circle(100% at 50% 50%)';
      notifiContent.style.pointerEvents = 'unset';
      audio_play(1);
      if (appWindow.isMinimized()) {
        appWindow.unminimize();
      }
    }
  }, 1000);
}
function timerStop() {
  clearInterval(timerCounter);
}

let notifiContent = document.getElementById('notifi-content')
let notifiBack = document.getElementById('notifi-back');
document
  .getElementById('notifi-check')
  .addEventListener('click', function () {
    notifiBack.style.opacity = '0'
    notifiContent.style.opacity = '0'
    notifiContent.style.clipPath = 'circle(0 at 50% 50%)';
    notifiBack.style.clipPath = 'circle(0 at 50% 50%)';
    notifiContent.style.pointerEvents = 'none';
  });

let tipbox = document.getElementById('tipbox');
window.tip_showup = function (isShow = true) {
  if (isShow) {
    tipbox.style.transform = 'translateY(0px)';
  }
  else {
    tipbox.style.transform = 'translateY(50px)';
  }
}
// tip_showup();
// await new Promise(resolve => setTimeout(resolve, 1000));
// tip_showup(false);

let data_canchange = false;

invoke('datatest_read').then(result => {
  if (result) {
    listbox.innerHTML = '';
    document.getElementById('Titlebar-Title').textContent = result[0];
    result[1].forEach(e => {
      task_add(e.description, e.completed == 'Completed');
      nameup(listbox.lastElementChild.children[1].dispatchEvent(new KeyboardEvent(
        'keydown', {
        key: 'Enter'
      }
      )));
    });
  }
  data_canchange = true
})

function data_save() {
  if (data_canchange) {
    console.log('Try to save');
    invoke('datatest_save', {
      title: document.getElementById('Titlebar-Title').textContent,
      tasks: Array.from(listbox.children).map(e => {
        return {
          description: e.children[1].value,
          completed: e.children[0].textContent == '1' ? 'Completed' : 'Wait'
        }
      })
    }).then(result => {
      console.log('Save result:' + result);
    })
  }
}