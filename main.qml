import QtQuick 2.14
import QtQuick.Controls 2.14
import QtQuick.Window 2.14
import QtQuick.Layouts 1.14
import QtQuick.Dialogs 1.3
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
        console.log("moved")
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
    property real pixelPerSecond: 10

    property string objectKey: "object"

    function timeMsForPosition(x) {
      return x / pixelPerSecond * 1000.0
    }

    function timecodeForMs(t) {
      return t / 1000.0 // todo
    }

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
        id: timeRulerScrollView
        height: timeline.rulerHeight
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
              x: timeline.pixelPerSecond * index
              y: index % 5 == 0 ? 10 : 20
              width: 1
              height: parent.height - (index % 5 == 0 ? 15 : 25)
              color: "gray"
            }
          }

          Repeater {
            model: 20

            Text {
              text: timeline.timecodeForMs(5 * index * 1000.0)
              x: timeline.pixelPerSecond * 5 * index + 2
              y: 0 
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

              property int layerId: index

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
                acceptedButtons: Qt.LeftButton | Qt.RightButton

                property real lastClickedX: -1

                onClicked: {
                  if (mouse.button == Qt.RightButton) {
                    layerContextMenu.popup()
                  }
                  lastClickedX = mouse.x
                }
                onPressAndHold: {
                  if (mouse.button == Qt.RightButton) {
                    layerContextMenu.popup()
                  }
                }

                Menu {
                  id: layerContextMenu
                  Menu {
                    title: qsTr("Add Media Object")
                    Action {
                      text: qsTr("Load File")
                      onTriggered: {
                        timelineOpenFileDialog.openDialog(layerId, layerMouseArea.lastClickedX)
                      }
                    }
                    Action {
                      text: qsTr("Text")
                      onTriggered: {
                        timelineConfigureTextDialog.openDialog("", layerId, layerMouseArea.lastClickedX)
                      }
                    }
                    // todo: implement
                  }
                  Action {
                    text: qsTr("Add Filter Object")
                    // todo: implement
                    onTriggered: {
                      timelineConfigureFilterDialog.openDialog("", layerId, layerMouseArea.lastClickedX)
                    }
                  }

                  MenuSeparator {}

                  // todo: implement other menu items
                }
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
                  drop.accept()

                  drop.source.y = parent.y
                  app.moveTimelineObject(drop.source.objectName, parent.layerId, timeline.timeMsForPosition(drop.source.x))
                  return Qt.MoveAction
                }
              }
            }
            
          }

          ListView {
            anchors.fill: parent
            model: app.objects
            delegate: timelineObject
          }

          // TimelineObject
          Component {
            id: timelineObject
            Rectangle {
              id: timelineObjectRect
              y: 2 * timeline.layerHeight
              x: startMs / 1000.0 * timeline.pixelPerSecond
              height: timeline.layerHeight
              width: durationMs / 1000.0 * timeline.pixelPerSecond
              gradient: Gradient {
                GradientStop { position: 0.0; color: "blue" }
                GradientStop { position: 1.0; color: "darkblue" }
              }

              // objectName in QML is not related to TimelineObject
              property string objectName: name 
              
              Text {
                color: "white"
                text: name
              }

              Drag.keys: [timeline.objectKey]
              Drag.active: objectMouseArea.drag.active
              Drag.supportedActions: Qt.MoveAction
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
                acceptedButtons: Qt.LeftButton | Qt.RightButton

                onReleased: parent.Drag.drop()

                onClicked: {
                  if (mouse.button == Qt.RightButton) {
                    objectContextMenu.popup()
                  }
                }

                Menu {
                  id: objectContextMenu
                  Action {
                    text: qsTr("Delete Object")
                    onTriggered: {
                      app.timelineRemoveObject(name)
                    }
                  }
                  Action {
                    text: qsTr("Apply Filter to Object...")
                    onTriggered: {
                      timelineApplyObjectFilterDialog.openDialog(name)
                    }
                  }
                }
              }

              Rectangle {
                id: objectInpointRect
                anchors.verticalCenter: parent.verticalCeter
                anchors.left: parent.left
                width: 10
                height: timeline.layerHeight
                color: "transparent"
                  
                MouseArea {
                  id: objectInpointMouseArea
                  anchors.fill: parent
                  cursorShape: Qt.SizeHorCursor	
                  
                  drag.target: parent
                  drag.axis: Drag.XAxis

                  onMouseXChanged: {
                    if (drag.active) {
                      // https://stackoverflow.com/a/29110791/3240599
                      parent.parent.width -= mouse.x
                      
                      let maxWidth = maxDurationMs / 1000.0 * timeline.pixelPerSecond
                      if (parent.parent.width > maxWidth) {
                        parent.parent.width = maxWidth
                        return
                      }
                      if (parent.parent.width < 20) {
                        parent.parent.width = 20
                        return
                      }

                      parent.parent.x += mouse.x
                    }
                  }

                  onReleased: {
                    if (drag.active) { // drag.active == false if it was just clicked
                      console.log("setting inpoint to: " + timeline.timeMsForPosition(parent.parent.x))
                      app.timelineChangeObjectInpoint(name, timeline.timeMsForPosition(parent.parent.x))
                    }
                  }
                }
              }

              Rectangle {
                id: objectOutpointRect
                anchors.verticalCenter: parent.verticalCeter
                anchors.right: parent.right
                width: 10
                height: timeline.layerHeight
                color: "transparent"
                  
                MouseArea {
                  id: objectOutpointMouseArea
                  anchors.fill: parent
                  cursorShape: Qt.SizeHorCursor	
                  
                  drag.target: parent
                  drag.axis: Drag.XAxis

                  onMouseXChanged: {
                    if (drag.active) {
                      // https://stackoverflow.com/a/29110791/3240599
                      parent.parent.width += mouse.x
                      
                      let maxWidth = maxDurationMs / 1000.0 * timeline.pixelPerSecond
                      if (parent.parent.width > maxWidth) {
                        parent.parent.width = maxWidth
                        return
                      }
                      if (parent.parent.width < 20) {
                        parent.parent.width = 20
                        return
                      }
                    }
                  }

                  onReleased: {
                    if (drag.active) { // drag.active == false if it was just clicked
                      console.log("setting outpoint to: " + timeline.timeMsForPosition(parent.parent.x + parent.parent.width))
                      app.timelineChangeObjectOutpoint(name, timeline.timeMsForPosition(parent.parent.x + parent.parent.width))
                    }
                  }
                }
              }
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

  FileDialog {
    id: timelineOpenFileDialog
    title: "Choose File..."
    folder: shortcuts.home
    visible: false

    property int layerId: -1
    property real x: -1

    onAccepted: {
      // rust_qt_binding_generator currently cannot bind QList<QUrl> arguments
      let urls = fileUrls.join("::::") // fixme: better splitter
      console.log(urls)
      app.timelineAddFileObject(urls, layerId, timeline.timeMsForPosition(x))
    }

    onRejected: {
      console.log("cancelled")
    }

    function openDialog(layerId, x) {
      this.layerId = layerId
      this.x = x
      this.open()
    }

    // onAccepted is read-only (make use of signals?)
    // function openWithCallback(cb) { // cb: function that takes a list<url>
    //   this.onAccepted = function() {
    //     cb(this.fileUrls)
    //   }
    //   this.open()
    // }
  }

  Dialog {
    id: timelineConfigureTextDialog
    title: "Configure Text Object..."
    visible: false
    standardButtons: StandardButton.Save | StandardButton.Cancel
    
    property string objectName: ""
    property int layerId: -1
    property real x: -1

    // todo: load existing information if objectName is designated
    // todo: prohibit empty object name
    // todo: change object name

    TextArea {
      id: textContentArea
      placeholderText: qsTr("Enter text here")
    }

    onAccepted: {
      app.timelineConfigureText(objectName, layerId, timeline.timeMsForPosition(x), textContentArea.text)
    }

    function openDialog(objectName, layerId, x) {
      this.objectName = objectName
      this.layerId = layerId
      this.x = x
      this.open()
    }
  }

  Dialog {
    id: timelineConfigureFilterDialog
    title: "Configure Filter Object..."
    visible: false
    standardButtons: StandardButton.Save | StandardButton.Cancel
    
    property string objectName: ""
    property int layerId: -1
    property real x: -1

    // todo: load existing information if objectName is designated
    // todo: prohibit empty object name
    // todo: change object name

    // TextArea {
    //   id: textContentArea
    //   placeholderText: qsTr("Enter text here")
    // }

    onAccepted: {
      app.timelineConfigureFilter(objectName, layerId, timeline.timeMsForPosition(x))
    }

    function openDialog(objectName, layerId, x) {
      this.objectName = objectName
      this.layerId = layerId
      this.x = x
      this.open()
    }
  }

  Dialog {
    id: timelineApplyObjectFilterDialog
    title: qsTr("Apply a Filter to Object")
    visible: false

    standardButtons: StandardButton.Save | StandardButton.Cancel
    
    property string objectName: ""

    // todo: load existing information if objectName is designated
    // todo: prohibit empty object name
    // todo: change object name

    // TextArea {
    //   id: textContentArea
    //   placeholderText: qsTr("Enter text here")
    // }

    onAccepted: {
      app.timelineApplyObjectFilter(objectName, "alpha method=blue")
    }

    function openDialog(objectName) {
      this.objectName = objectName
      this.open()
    }
  }
}