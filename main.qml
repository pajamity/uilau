import QtQuick 2.14
import QtQuick.Controls 2.14
import QtQuick.Window 2.14
import QtQuick.Layouts 1.14
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

  Window {
    id: timeline
    visible: true
    width: 640
    height: 400
    title: qsTr("timeline")

    property int rulerHeight: 40
    property int layerHeight: 40

    property string objectKey: "object"

    GridLayout { // todo: vs. GridLayout?
      id: grid
      columns: 2
      anchors.fill: parent

      Item {
        id: leftTop
        height: timeline.rulerHeight
        width: 70
        Text {
          text: "Root"
        }

        // todo: scaling
        // Slider {
        //   value: 1
        //   from: 0
        //   to: 10
        // }
      }

      ScrollView {
        Layout.fillWidth: true
        height: timeline.rulerHeight
        id: timeRulerScrollView
        clip: true
        ScrollBar.horizontal: horizontalScrollBar

        Item {
          height: parent.height
          implicitHeight: parent.height
          implicitWidth: 1000 // todo
          
          // todo
          // ListView {
          Repeater {
            model: 100
            
            Rectangle {
              x: 10 * index
              y: index % 5 == 0 ? 10 : 20
              width: 1
              height: parent.height - (index % 5 == 0 ? 15 : 25)
              color: "gray"
            }
          }

          Rectangle {
            height: 1
            width: 1000
            color: "black"
            y: parent.height - 5
          }
        }
      }

      ScrollView {
        id: layerListScrollView
        width: leftTop.width
        Layout.fillHeight: true
        clip: true

        // ScrollBar.vertical.policy: ScrollBar.AlwaysOff
        // ScrollBar.vertical.interactive: true
        ScrollBar.vertical: verticalScrollBar
        // fixme: scrollbars are way too fast

        // todo
        // ListView {
        Item {
          implicitHeight: timeline.layerHeight * 100
          implicitWidth: 70

          Repeater {
            model: 100
            Rectangle {
              x: parent.x
              y: parent.y + timeline.layerHeight * index + 1
              width: 70
              height: timeline.layerHeight - 2
              border.color: "black"
              border.width: 1

              Text {
                anchors.centerIn: parent
                anchors.fill: parent
                horizontalAlignment: Text.AlignHCenter
                verticalAlignment: Text.AlignVCenter
                text: "Layer " + index
              }
            }
          }
        }
      }

      ScrollView {
        id: trackAreaScrollView
        Layout.fillWidth: true
        Layout.fillHeight: true
        clip: true

        ScrollBar.horizontal: horizontalScrollBar
        ScrollBar.vertical: verticalScrollBar
        
        Item {
          implicitHeight: timeline.layerHeight * 100
          implicitWidth: 1000 // todo

          // ListView {
          Repeater {
            id: layerListView
            model: 100

            // delegate: Rectangle {
            Rectangle {
              id: layerRect
              width: 1000 // todo
              height: timeline.layerHeight
              y: index * timeline.layerHeight
              color: layerMouseArea.containsMouse ? "lightgray" : "white"

              Rectangle {
                y: index * timeline.layerHeight
                width: 1000
                height: 1
                color: "gray"
              }

              MouseArea {
                id: layerMouseArea
                hoverEnabled: true
                anchors.fill: parent
              }

              DropArea {
                id: layerDragTarget
                anchors.fill: parent

                keys: [timeline.objectKey]
                states: State {
                  when: layerDragTarget.containsDrag
                  PropertyChanges {
                    target: layerRect
                    color: "lightgray"
                  }
                }

                onDropped: {
                  console.log(drop)
                  
                }
              }
            }
            
          }

          // TimelineObject
          Rectangle {
            id: timelineObject1
            y: 2 * timeline.layerHeight
            x: 100
            height: timeline.layerHeight
            width: 300
            gradient: Gradient {
              GradientStop { position: 0.0; color: "blue" }
              GradientStop { position: 1.0; color: "darkblue" }
            }

            Text {
              color: "white"
              text: "Sample Object"
            }

            Drag.keys: [timeline.objectKey]
            Drag.active: objectMouseArea.drag.active
            Drag.hotSpot.x: 10
            Drag.hotSpot.y: 10
            states: State {
              when: objectMouseArea.drag.active
              // AnchorChanges {
              //   target: timelineObject1
              // }
            }

            MouseArea {
              id: objectMouseArea
              anchors.fill: parent
              drag.target: parent
            }
          }
        }
      }
    }

    ScrollBar {
      id: verticalScrollBar
      height: parent.height
      anchors.right: parent.right
      policy: ScrollBar.AlwaysOff // fixme: AlwaysOn shows weird scrollbar
    }

    ScrollBar {
      id: horizontalScrollBar
      width: parent.width
      anchors.bottom: parent.bottom
      policy: ScrollBar.AlwaysOff
    }
  }
}