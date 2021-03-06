/* generated by rust_qt_binding_generator */
#include "Bindings.h"

namespace {

    struct option_quintptr {
    public:
        quintptr value;
        bool some;
        operator QVariant() const {
            if (some) {
                return QVariant::fromValue(value);
            }
            return QVariant();
        }
    };
    static_assert(std::is_pod<option_quintptr>::value, "option_quintptr must be a POD type.");

    typedef void (*qstring_set)(QString* val, const char* utf8, int nbytes);
    void set_qstring(QString* val, const char* utf8, int nbytes) {
        *val = QString::fromUtf8(utf8, nbytes);
    }

    struct qmodelindex_t {
        int row;
        quintptr id;
    };
    inline QVariant cleanNullQVariant(const QVariant& v) {
        return (v.isNull()) ?QVariant() :v;
    }
    inline void appCanvasHeightChanged(App* o)
    {
        Q_EMIT o->canvasHeightChanged();
    }
    inline void appCanvasWidthChanged(App* o)
    {
        Q_EMIT o->canvasWidthChanged();
    }
    inline void appDurationMsChanged(App* o)
    {
        Q_EMIT o->durationMsChanged();
    }
    inline void appPlayingChanged(App* o)
    {
        Q_EMIT o->playingChanged();
    }
    inline void appPositionMsChanged(App* o)
    {
        Q_EMIT o->positionMsChanged();
    }
}
extern "C" {
    App::Private* app_new(App*, void (*)(App*), void (*)(App*), void (*)(App*), Layers*,
        void (*)(const Layers*),
        void (*)(Layers*),
        void (*)(Layers*),
        void (*)(Layers*, quintptr, quintptr),
        void (*)(Layers*),
        void (*)(Layers*),
        void (*)(Layers*, int, int),
        void (*)(Layers*),
        void (*)(Layers*, int, int, int),
        void (*)(Layers*),
        void (*)(Layers*, int, int),
        void (*)(Layers*), TimelineObjects*,
        void (*)(const TimelineObjects*),
        void (*)(TimelineObjects*),
        void (*)(TimelineObjects*),
        void (*)(TimelineObjects*, quintptr, quintptr),
        void (*)(TimelineObjects*),
        void (*)(TimelineObjects*),
        void (*)(TimelineObjects*, int, int),
        void (*)(TimelineObjects*),
        void (*)(TimelineObjects*, int, int, int),
        void (*)(TimelineObjects*),
        void (*)(TimelineObjects*, int, int),
        void (*)(TimelineObjects*), void (*)(App*), void (*)(App*));
    void app_free(App::Private*);
    quint64 app_canvas_height_get(const App::Private*);
    quint64 app_canvas_width_get(const App::Private*);
    quint64 app_duration_ms_get(const App::Private*);
    Layers::Private* app_layers_get(const App::Private*);
    TimelineObjects::Private* app_objects_get(const App::Private*);
    bool app_playing_get(const App::Private*);
    quint64 app_position_ms_get(const App::Private*);
    void app_move_timeline_object(App::Private*, const ushort*, int, quint64, float);
    void app_pause(App::Private*);
    void app_play(App::Private*);
    void app_seek_to(App::Private*, quint64);
    void app_timeline_add_file_object(App::Private*, const ushort*, int, quint64, float);
    void app_timeline_apply_object_filter(App::Private*, const ushort*, int, const ushort*, int);
    void app_timeline_change_object_inpoint(App::Private*, const ushort*, int, float);
    void app_timeline_change_object_outpoint(App::Private*, const ushort*, int, float);
    void app_timeline_configure_filter(App::Private*, const ushort*, int, quint64, float);
    void app_timeline_configure_text(App::Private*, const ushort*, int, quint64, float, const ushort*, int);
    void app_timeline_remove_object(App::Private*, const ushort*, int);
    void app_timeline_set_object_x(App::Private*, const ushort*, int, qint64);
};

extern "C" {
    void layers_sort(Layers::Private*, unsigned char column, Qt::SortOrder order = Qt::AscendingOrder);

    int layers_row_count(const Layers::Private*);
    bool layers_insert_rows(Layers::Private*, int, int);
    bool layers_remove_rows(Layers::Private*, int, int);
    bool layers_can_fetch_more(const Layers::Private*);
    void layers_fetch_more(Layers::Private*);
}
int Layers::columnCount(const QModelIndex &parent) const
{
    return (parent.isValid()) ? 0 : 1;
}

bool Layers::hasChildren(const QModelIndex &parent) const
{
    return rowCount(parent) > 0;
}

int Layers::rowCount(const QModelIndex &parent) const
{
    return (parent.isValid()) ? 0 : layers_row_count(m_d);
}

bool Layers::insertRows(int row, int count, const QModelIndex &)
{
    return layers_insert_rows(m_d, row, count);
}

bool Layers::removeRows(int row, int count, const QModelIndex &)
{
    return layers_remove_rows(m_d, row, count);
}

QModelIndex Layers::index(int row, int column, const QModelIndex &parent) const
{
    if (!parent.isValid() && row >= 0 && row < rowCount(parent) && column >= 0 && column < 1) {
        return createIndex(row, column, (quintptr)row);
    }
    return QModelIndex();
}

QModelIndex Layers::parent(const QModelIndex &) const
{
    return QModelIndex();
}

bool Layers::canFetchMore(const QModelIndex &parent) const
{
    return (parent.isValid()) ? 0 : layers_can_fetch_more(m_d);
}

void Layers::fetchMore(const QModelIndex &parent)
{
    if (!parent.isValid()) {
        layers_fetch_more(m_d);
    }
}
void Layers::updatePersistentIndexes() {}

void Layers::sort(int column, Qt::SortOrder order)
{
    layers_sort(m_d, column, order);
}
Qt::ItemFlags Layers::flags(const QModelIndex &i) const
{
    auto flags = QAbstractItemModel::flags(i);
    return flags;
}

QVariant Layers::data(const QModelIndex &index, int role) const
{
    Q_ASSERT(rowCount(index.parent()) > index.row());
    switch (index.column()) {
    case 0:
        switch (role) {
        }
        break;
    }
    return QVariant();
}

int Layers::role(const char* name) const {
    auto names = roleNames();
    auto i = names.constBegin();
    while (i != names.constEnd()) {
        if (i.value() == name) {
            return i.key();
        }
        ++i;
    }
    return -1;
}
QHash<int, QByteArray> Layers::roleNames() const {
    QHash<int, QByteArray> names = QAbstractItemModel::roleNames();
    return names;
}
QVariant Layers::headerData(int section, Qt::Orientation orientation, int role) const
{
    if (orientation != Qt::Horizontal) {
        return QVariant();
    }
    return m_headerData.value(qMakePair(section, (Qt::ItemDataRole)role), role == Qt::DisplayRole ?QString::number(section + 1) :QVariant());
}

bool Layers::setHeaderData(int section, Qt::Orientation orientation, const QVariant &value, int role)
{
    if (orientation != Qt::Horizontal) {
        return false;
    }
    m_headerData.insert(qMakePair(section, (Qt::ItemDataRole)role), value);
    return true;
}

extern "C" {
    Layers::Private* layers_new(Layers*,
        void (*)(const Layers*),
        void (*)(Layers*),
        void (*)(Layers*),
        void (*)(Layers*, quintptr, quintptr),
        void (*)(Layers*),
        void (*)(Layers*),
        void (*)(Layers*, int, int),
        void (*)(Layers*),
        void (*)(Layers*, int, int, int),
        void (*)(Layers*),
        void (*)(Layers*, int, int),
        void (*)(Layers*));
    void layers_free(Layers::Private*);
};

extern "C" {
    quint64 timeline_objects_data_duration_ms(const TimelineObjects::Private*, int);
    void timeline_objects_data_kind(const TimelineObjects::Private*, int, QString*, qstring_set);
    quint64 timeline_objects_data_layer_id(const TimelineObjects::Private*, int);
    quint64 timeline_objects_data_max_duration_ms(const TimelineObjects::Private*, int);
    void timeline_objects_data_name(const TimelineObjects::Private*, int, QString*, qstring_set);
    quint64 timeline_objects_data_start_ms(const TimelineObjects::Private*, int);
    void timeline_objects_sort(TimelineObjects::Private*, unsigned char column, Qt::SortOrder order = Qt::AscendingOrder);

    int timeline_objects_row_count(const TimelineObjects::Private*);
    bool timeline_objects_insert_rows(TimelineObjects::Private*, int, int);
    bool timeline_objects_remove_rows(TimelineObjects::Private*, int, int);
    bool timeline_objects_can_fetch_more(const TimelineObjects::Private*);
    void timeline_objects_fetch_more(TimelineObjects::Private*);
}
int TimelineObjects::columnCount(const QModelIndex &parent) const
{
    return (parent.isValid()) ? 0 : 1;
}

bool TimelineObjects::hasChildren(const QModelIndex &parent) const
{
    return rowCount(parent) > 0;
}

int TimelineObjects::rowCount(const QModelIndex &parent) const
{
    return (parent.isValid()) ? 0 : timeline_objects_row_count(m_d);
}

bool TimelineObjects::insertRows(int row, int count, const QModelIndex &)
{
    return timeline_objects_insert_rows(m_d, row, count);
}

bool TimelineObjects::removeRows(int row, int count, const QModelIndex &)
{
    return timeline_objects_remove_rows(m_d, row, count);
}

QModelIndex TimelineObjects::index(int row, int column, const QModelIndex &parent) const
{
    if (!parent.isValid() && row >= 0 && row < rowCount(parent) && column >= 0 && column < 1) {
        return createIndex(row, column, (quintptr)row);
    }
    return QModelIndex();
}

QModelIndex TimelineObjects::parent(const QModelIndex &) const
{
    return QModelIndex();
}

bool TimelineObjects::canFetchMore(const QModelIndex &parent) const
{
    return (parent.isValid()) ? 0 : timeline_objects_can_fetch_more(m_d);
}

void TimelineObjects::fetchMore(const QModelIndex &parent)
{
    if (!parent.isValid()) {
        timeline_objects_fetch_more(m_d);
    }
}
void TimelineObjects::updatePersistentIndexes() {}

void TimelineObjects::sort(int column, Qt::SortOrder order)
{
    timeline_objects_sort(m_d, column, order);
}
Qt::ItemFlags TimelineObjects::flags(const QModelIndex &i) const
{
    auto flags = QAbstractItemModel::flags(i);
    return flags;
}

quint64 TimelineObjects::durationMs(int row) const
{
    return timeline_objects_data_duration_ms(m_d, row);
}

QString TimelineObjects::kind(int row) const
{
    QString s;
    timeline_objects_data_kind(m_d, row, &s, set_qstring);
    return s;
}

quint64 TimelineObjects::layerId(int row) const
{
    return timeline_objects_data_layer_id(m_d, row);
}

quint64 TimelineObjects::maxDurationMs(int row) const
{
    return timeline_objects_data_max_duration_ms(m_d, row);
}

QString TimelineObjects::name(int row) const
{
    QString s;
    timeline_objects_data_name(m_d, row, &s, set_qstring);
    return s;
}

quint64 TimelineObjects::startMs(int row) const
{
    return timeline_objects_data_start_ms(m_d, row);
}

QVariant TimelineObjects::data(const QModelIndex &index, int role) const
{
    Q_ASSERT(rowCount(index.parent()) > index.row());
    switch (index.column()) {
    case 0:
        switch (role) {
        case Qt::UserRole + 0:
            return QVariant::fromValue(durationMs(index.row()));
        case Qt::UserRole + 1:
            return QVariant::fromValue(kind(index.row()));
        case Qt::UserRole + 2:
            return QVariant::fromValue(layerId(index.row()));
        case Qt::UserRole + 3:
            return QVariant::fromValue(maxDurationMs(index.row()));
        case Qt::UserRole + 4:
            return QVariant::fromValue(name(index.row()));
        case Qt::UserRole + 5:
            return QVariant::fromValue(startMs(index.row()));
        }
        break;
    }
    return QVariant();
}

int TimelineObjects::role(const char* name) const {
    auto names = roleNames();
    auto i = names.constBegin();
    while (i != names.constEnd()) {
        if (i.value() == name) {
            return i.key();
        }
        ++i;
    }
    return -1;
}
QHash<int, QByteArray> TimelineObjects::roleNames() const {
    QHash<int, QByteArray> names = QAbstractItemModel::roleNames();
    names.insert(Qt::UserRole + 0, "durationMs");
    names.insert(Qt::UserRole + 1, "kind");
    names.insert(Qt::UserRole + 2, "layerId");
    names.insert(Qt::UserRole + 3, "maxDurationMs");
    names.insert(Qt::UserRole + 4, "name");
    names.insert(Qt::UserRole + 5, "startMs");
    return names;
}
QVariant TimelineObjects::headerData(int section, Qt::Orientation orientation, int role) const
{
    if (orientation != Qt::Horizontal) {
        return QVariant();
    }
    return m_headerData.value(qMakePair(section, (Qt::ItemDataRole)role), role == Qt::DisplayRole ?QString::number(section + 1) :QVariant());
}

bool TimelineObjects::setHeaderData(int section, Qt::Orientation orientation, const QVariant &value, int role)
{
    if (orientation != Qt::Horizontal) {
        return false;
    }
    m_headerData.insert(qMakePair(section, (Qt::ItemDataRole)role), value);
    return true;
}

extern "C" {
    TimelineObjects::Private* timeline_objects_new(TimelineObjects*,
        void (*)(const TimelineObjects*),
        void (*)(TimelineObjects*),
        void (*)(TimelineObjects*),
        void (*)(TimelineObjects*, quintptr, quintptr),
        void (*)(TimelineObjects*),
        void (*)(TimelineObjects*),
        void (*)(TimelineObjects*, int, int),
        void (*)(TimelineObjects*),
        void (*)(TimelineObjects*, int, int, int),
        void (*)(TimelineObjects*),
        void (*)(TimelineObjects*, int, int),
        void (*)(TimelineObjects*));
    void timeline_objects_free(TimelineObjects::Private*);
};

App::App(bool /*owned*/, QObject *parent):
    QObject(parent),
    m_layers(new Layers(false, this)),
    m_objects(new TimelineObjects(false, this)),
    m_d(nullptr),
    m_ownsPrivate(false)
{
}

App::App(QObject *parent):
    QObject(parent),
    m_layers(new Layers(false, this)),
    m_objects(new TimelineObjects(false, this)),
    m_d(app_new(this,
        appCanvasHeightChanged,
        appCanvasWidthChanged,
        appDurationMsChanged, m_layers,
        [](const Layers* o) {
            Q_EMIT o->newDataReady(QModelIndex());
        },
        [](Layers* o) {
            Q_EMIT o->layoutAboutToBeChanged();
        },
        [](Layers* o) {
            o->updatePersistentIndexes();
            Q_EMIT o->layoutChanged();
        },
        [](Layers* o, quintptr first, quintptr last) {
            o->dataChanged(o->createIndex(first, 0, first),
                       o->createIndex(last, 0, last));
        },
        [](Layers* o) {
            o->beginResetModel();
        },
        [](Layers* o) {
            o->endResetModel();
        },
        [](Layers* o, int first, int last) {
            o->beginInsertRows(QModelIndex(), first, last);
        },
        [](Layers* o) {
            o->endInsertRows();
        },
        [](Layers* o, int first, int last, int destination) {
            o->beginMoveRows(QModelIndex(), first, last, QModelIndex(), destination);
        },
        [](Layers* o) {
            o->endMoveRows();
        },
        [](Layers* o, int first, int last) {
            o->beginRemoveRows(QModelIndex(), first, last);
        },
        [](Layers* o) {
            o->endRemoveRows();
        }
, m_objects,
        [](const TimelineObjects* o) {
            Q_EMIT o->newDataReady(QModelIndex());
        },
        [](TimelineObjects* o) {
            Q_EMIT o->layoutAboutToBeChanged();
        },
        [](TimelineObjects* o) {
            o->updatePersistentIndexes();
            Q_EMIT o->layoutChanged();
        },
        [](TimelineObjects* o, quintptr first, quintptr last) {
            o->dataChanged(o->createIndex(first, 0, first),
                       o->createIndex(last, 0, last));
        },
        [](TimelineObjects* o) {
            o->beginResetModel();
        },
        [](TimelineObjects* o) {
            o->endResetModel();
        },
        [](TimelineObjects* o, int first, int last) {
            o->beginInsertRows(QModelIndex(), first, last);
        },
        [](TimelineObjects* o) {
            o->endInsertRows();
        },
        [](TimelineObjects* o, int first, int last, int destination) {
            o->beginMoveRows(QModelIndex(), first, last, QModelIndex(), destination);
        },
        [](TimelineObjects* o) {
            o->endMoveRows();
        },
        [](TimelineObjects* o, int first, int last) {
            o->beginRemoveRows(QModelIndex(), first, last);
        },
        [](TimelineObjects* o) {
            o->endRemoveRows();
        }
,
        appPlayingChanged,
        appPositionMsChanged)),
    m_ownsPrivate(true)
{
    m_layers->m_d = app_layers_get(m_d);
    m_objects->m_d = app_objects_get(m_d);
    connect(this->m_layers, &Layers::newDataReady, this->m_layers, [this](const QModelIndex& i) {
        this->m_layers->fetchMore(i);
    }, Qt::QueuedConnection);
    connect(this->m_objects, &TimelineObjects::newDataReady, this->m_objects, [this](const QModelIndex& i) {
        this->m_objects->fetchMore(i);
    }, Qt::QueuedConnection);
}

App::~App() {
    if (m_ownsPrivate) {
        app_free(m_d);
    }
}
quint64 App::canvasHeight() const
{
    return app_canvas_height_get(m_d);
}
quint64 App::canvasWidth() const
{
    return app_canvas_width_get(m_d);
}
quint64 App::durationMs() const
{
    return app_duration_ms_get(m_d);
}
const Layers* App::layers() const
{
    return m_layers;
}
Layers* App::layers()
{
    return m_layers;
}
const TimelineObjects* App::objects() const
{
    return m_objects;
}
TimelineObjects* App::objects()
{
    return m_objects;
}
bool App::playing() const
{
    return app_playing_get(m_d);
}
quint64 App::positionMs() const
{
    return app_position_ms_get(m_d);
}
void App::moveTimelineObject(const QString& obj_name, quint64 dst_layer_id, float dst_time_ms)
{
    return app_move_timeline_object(m_d, obj_name.utf16(), obj_name.size(), dst_layer_id, dst_time_ms);
}
void App::pause()
{
    return app_pause(m_d);
}
void App::play()
{
    return app_play(m_d);
}
void App::seekTo(quint64 to)
{
    return app_seek_to(m_d, to);
}
void App::timelineAddFileObject(const QString& file_urls, quint64 dst_layer_id, float dst_time_ms)
{
    return app_timeline_add_file_object(m_d, file_urls.utf16(), file_urls.size(), dst_layer_id, dst_time_ms);
}
void App::timelineApplyObjectFilter(const QString& obj_name, const QString& description)
{
    return app_timeline_apply_object_filter(m_d, obj_name.utf16(), obj_name.size(), description.utf16(), description.size());
}
void App::timelineChangeObjectInpoint(const QString& obj_name, float inpoint_ms)
{
    return app_timeline_change_object_inpoint(m_d, obj_name.utf16(), obj_name.size(), inpoint_ms);
}
void App::timelineChangeObjectOutpoint(const QString& obj_name, float outpoint_ms)
{
    return app_timeline_change_object_outpoint(m_d, obj_name.utf16(), obj_name.size(), outpoint_ms);
}
void App::timelineConfigureFilter(const QString& obj_name, quint64 dst_layer_id, float dst_time_ms)
{
    return app_timeline_configure_filter(m_d, obj_name.utf16(), obj_name.size(), dst_layer_id, dst_time_ms);
}
void App::timelineConfigureText(const QString& obj_name, quint64 dst_layer_id, float dst_time_ms, const QString& text)
{
    return app_timeline_configure_text(m_d, obj_name.utf16(), obj_name.size(), dst_layer_id, dst_time_ms, text.utf16(), text.size());
}
void App::timelineRemoveObject(const QString& obj_name)
{
    return app_timeline_remove_object(m_d, obj_name.utf16(), obj_name.size());
}
void App::timelineSetObjectX(const QString& obj_name, qint64 x)
{
    return app_timeline_set_object_x(m_d, obj_name.utf16(), obj_name.size(), x);
}
Layers::Layers(bool /*owned*/, QObject *parent):
    QAbstractItemModel(parent),
    m_d(nullptr),
    m_ownsPrivate(false)
{
    initHeaderData();
}

Layers::Layers(QObject *parent):
    QAbstractItemModel(parent),
    m_d(layers_new(this,
        [](const Layers* o) {
            Q_EMIT o->newDataReady(QModelIndex());
        },
        [](Layers* o) {
            Q_EMIT o->layoutAboutToBeChanged();
        },
        [](Layers* o) {
            o->updatePersistentIndexes();
            Q_EMIT o->layoutChanged();
        },
        [](Layers* o, quintptr first, quintptr last) {
            o->dataChanged(o->createIndex(first, 0, first),
                       o->createIndex(last, 0, last));
        },
        [](Layers* o) {
            o->beginResetModel();
        },
        [](Layers* o) {
            o->endResetModel();
        },
        [](Layers* o, int first, int last) {
            o->beginInsertRows(QModelIndex(), first, last);
        },
        [](Layers* o) {
            o->endInsertRows();
        },
        [](Layers* o, int first, int last, int destination) {
            o->beginMoveRows(QModelIndex(), first, last, QModelIndex(), destination);
        },
        [](Layers* o) {
            o->endMoveRows();
        },
        [](Layers* o, int first, int last) {
            o->beginRemoveRows(QModelIndex(), first, last);
        },
        [](Layers* o) {
            o->endRemoveRows();
        }
)),
    m_ownsPrivate(true)
{
    connect(this, &Layers::newDataReady, this, [this](const QModelIndex& i) {
        this->fetchMore(i);
    }, Qt::QueuedConnection);
    initHeaderData();
}

Layers::~Layers() {
    if (m_ownsPrivate) {
        layers_free(m_d);
    }
}
void Layers::initHeaderData() {
}
TimelineObjects::TimelineObjects(bool /*owned*/, QObject *parent):
    QAbstractItemModel(parent),
    m_d(nullptr),
    m_ownsPrivate(false)
{
    initHeaderData();
}

TimelineObjects::TimelineObjects(QObject *parent):
    QAbstractItemModel(parent),
    m_d(timeline_objects_new(this,
        [](const TimelineObjects* o) {
            Q_EMIT o->newDataReady(QModelIndex());
        },
        [](TimelineObjects* o) {
            Q_EMIT o->layoutAboutToBeChanged();
        },
        [](TimelineObjects* o) {
            o->updatePersistentIndexes();
            Q_EMIT o->layoutChanged();
        },
        [](TimelineObjects* o, quintptr first, quintptr last) {
            o->dataChanged(o->createIndex(first, 0, first),
                       o->createIndex(last, 0, last));
        },
        [](TimelineObjects* o) {
            o->beginResetModel();
        },
        [](TimelineObjects* o) {
            o->endResetModel();
        },
        [](TimelineObjects* o, int first, int last) {
            o->beginInsertRows(QModelIndex(), first, last);
        },
        [](TimelineObjects* o) {
            o->endInsertRows();
        },
        [](TimelineObjects* o, int first, int last, int destination) {
            o->beginMoveRows(QModelIndex(), first, last, QModelIndex(), destination);
        },
        [](TimelineObjects* o) {
            o->endMoveRows();
        },
        [](TimelineObjects* o, int first, int last) {
            o->beginRemoveRows(QModelIndex(), first, last);
        },
        [](TimelineObjects* o) {
            o->endRemoveRows();
        }
)),
    m_ownsPrivate(true)
{
    connect(this, &TimelineObjects::newDataReady, this, [this](const QModelIndex& i) {
        this->fetchMore(i);
    }, Qt::QueuedConnection);
    initHeaderData();
}

TimelineObjects::~TimelineObjects() {
    if (m_ownsPrivate) {
        timeline_objects_free(m_d);
    }
}
void TimelineObjects::initHeaderData() {
}
