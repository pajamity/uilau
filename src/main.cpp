#include "Bindings.h"

#include <QtCore/QFile>
#include <QtGui/QGuiApplication>
#include <QtQml/QQmlApplicationEngine>
#include <QtQuick/QQuickItem>
#include <QtQuick/QQuickWindow>
#include <QtQml/qqml.h>

#include <iostream>
#include <glib-object.h>

// see also: pajamity/gstreamer-rs-qt-player

// exported functions
extern "C" {
// functions C++ source exports (and Rust source calls them)
int main_cpp(const char* app);
void set_widget_to_sink(void *sink, QQuickItem *videoItem);

// functions Rust source exports (so C++ source calls it)
void set_video_item_pointer(QQuickItem *videoItem);
}

int main_cpp(const char* appPath) {
    int argc = 1;
    char* argv[1] = { (char*)appPath };
    QGuiApplication app(argc, argv);
    qmlRegisterType<App>("RustCode", 1, 0, "App");

    QQmlApplicationEngine engine;
    if (QFile("main.qml").exists()) {
        engine.load(QUrl(QStringLiteral("main.qml")));
    } else {
        engine.load(QUrl(QStringLiteral("qrc:/main.qml")));
    }
    if (engine.rootObjects().isEmpty())
        return -1;

    QQuickWindow *rootObject = static_cast<QQuickWindow *>(engine.rootObjects().first());
    QQuickItem *videoItem = rootObject->findChild<QQuickItem *>("videoItem");
    set_video_item_pointer(videoItem);
    std::cout << "Passed the address of videoItem to Rust: " << videoItem << std::endl;

    return app.exec(); // This starts an event loop so we won't return till that loop ends.
}

void set_widget_to_sink(void *sink, QQuickItem *videoItem) {
    std::cout << "Address of sink C++ was given by Rust: " << sink << std::endl;
    std::cout << "Address of videoItem C++ was given by Rust: " << videoItem << std::endl;
    g_object_set(sink, "widget", videoItem, NULL);
}