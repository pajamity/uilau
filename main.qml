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

  Player {
    id: player
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

    Button {
      id: playPauseButton
      objectName: "playPauseButton"
      width: 80
      height: 30
      text: "Play"

      property bool playing: false

      function play() {
        playpause.playing = true
        playpause.text = "Pause"
        player.play()
      }

      function pause() {
        playpause.playing = false
        playpause.text = "Play"
        player.pause()
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

      // todo: implement
    }

    Button {
      id: seekLastFrameButton
      objectName: "seekLastFrameButton"

      // todo: implement
    }
  }
}
