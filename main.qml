import QtQuick 2.6
import QtQuick.Controls 2.15
import QtQuick.Window 2.2

ApplicationWindow {
    visible: true
    width: 640
    height: 480
    title: qsTr("uilau")

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
}
