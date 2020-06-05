import QtQuick 2.14
import QtQuick.Controls 2.14
import QtQuick.Window 2.14
import RustCode 1.0

import org.freedesktop.gstreamer.GLVideoItem 1.0

ApplicationWindow {
  visible: true
  width: 640
  height: 480
  title: qsTr("uilau")

  App {
    id: app
  } 

  menuBar: MenuBar {
    Menu {
      title: qsTr("File")
      
      Action { text: qsTr("Open") }
      Action { text: qsTr("Open Recent") }
      Action { text: qsTr("Close") }
      Action { text: qsTr("Open As Object") }
      Action { text: qsTr("Open Audio File") }

      Action { text: qsTr("Save") }
      Action { text: qsTr("Import") }
      Action { text: qsTr("Export") }

      Action { text: qsTr("Preferences") }

      Action { text: qsTr("Quit") }
    }

    Menu {
      title: qsTr("Misc")

      Action {
        text: qsTr("File Info")
      }
      Action {
        text: qsTr("Overlay Info")
      }
      Action {
        text: qsTr("Plugin Info")
      }
      Action {
        text: qsTr("About uilau")
        onTriggered: aboutPopup.open()
      }
    }
  }

  Item {
    anchors.fill: parent

    GstGLVideoItem {
      id: videoItem
      objectName: "videoItem"
      anchors.centerIn: parent
      width: parent.width
      height: parent.height
    }
  }

  Row {
    anchors.bottom: parent.bottom
    height: 30

    Button {
      id: playPauseButton
      objectName: "playPauseButton"
      width: 80
      height: 30
      text: "Play"

      property bool playing: false

      function play() {
        this.playing = true
        this.text = "Pause"
        previewSliderTimer.start()
        app.play()
      }

      function pause() {
        this.playing = false
        this.text = "Play"
        previewSliderTimer.stop()
        app.pause()
      }
      
      onClicked: {
        if (playing) {
          pause()
        } else {
          play()
        }
      }
    }

    Button {
      id: seekFirstFrameButton
      objectName: "seekFirstFrameButton"
      width: 80
      height: 30
      text: "First"

      onClicked: {
        app.seekTo(0)
      }
    }

    Slider {
      id: previewSlider
      objectName: "previewSlider"
      width: 500
      height: 30

      from: 0
      to: app.durationMs 
      value: app.positionMs

      onMoved: {
        app.seekTo(this.value)
      }
    }

    Timer {
      id: previewSliderTimer
      interval: 500
      repeat: true
      running: false

      onTriggered: {
        previewSlider.value = app.positionMs
        previewSlider.to = app.durationMs // todo: run only when needed
      }
    }

    Button {
      id: seekLastFrameButton
      objectName: "seekLastFrameButton"
      width: 80
      height: 30
      text: "Last"

      onClicked: {
        app.seekTo(app.durationMs)
      }
    }
  }

  Popup {
    id: aboutPopup
    anchors.centerIn: parent
    width: 200
    height: 150
    modal: false
    focus: true
    
    contentItem: Text {
      text: qsTr("uilau beta\n")
    }
  }
}
