//! Headless UI tests of the minimal interface (`FilmcraftApp::with_minimal_ui`, `?minimal` on the
//! web): it hides the Discord button, the Tools panel, the monitor buttons that have shortcuts and
//! the History, Essential Graphics and Text tabs, and slims the timeline's track headers. Off by
//! default, where nothing changes.

use egui_kittest::Harness;
use filmcraft_engine::Session;
use filmcraft_ui_egui::FilmcraftApp;
use filmcraft_ui_egui::dock::PanelKind;
use serde_json::json;

fn harness(minimal: bool) -> Harness<'static, FilmcraftApp> {
    let mut s = Session::default();
    s.execute("file.openDemoProject", json!({})).unwrap();
    let app = FilmcraftApp::new(s).with_minimal_ui(minimal);
    let mut harness = Harness::builder().with_size(egui::vec2(1600.0, 980.0)).with_max_steps(10_000).build_eframe(move |_cc| app);
    for _ in 0..6 {
        harness.step();
    }
    harness
}

fn has(h: &Harness<'static, FilmcraftApp>, prefix: &str) -> bool {
    !h.state().auto.query(prefix).is_empty()
}

#[test]
fn the_default_interface_is_unchanged() {
    let h = harness(false);
    assert!(has(&h, "header.discord"));
    assert!(has(&h, "tools."), "the Tools panel is shown");
    assert!(has(&h, "program.transport.markers.add"));
    assert!(has(&h, "program.transport.sequence.lift"));
    assert!(has(&h, "program.transport.editor"));
    assert!(has(&h, "timeline.track.V1.sourcePatch"));
    assert!(has(&h, "timeline.track.V1.syncLock"));
    assert!(h.state().ui.dock.contains(PanelKind::History));
    assert!(h.state().ui.dock.contains(PanelKind::EssentialGraphics));
    assert!(h.state().ui.dock.contains(PanelKind::Text));
    assert!(h.state().ui_error.is_none(), "{:?}", h.state().ui_error);
}

#[test]
fn the_minimal_interface_drops_the_buttons_that_have_shortcuts() {
    let h = harness(true);
    assert!(!has(&h, "header.discord"), "Discord button hidden");
    assert!(!has(&h, "tools."), "Tools panel hidden");
    assert!(!has(&h, "program.transport.markers.add"));
    assert!(!has(&h, "program.transport.markers.markIn"));
    assert!(!has(&h, "program.transport.sequence.lift"));
    assert!(!has(&h, "program.transport.sequence.extract"));
    assert!(!has(&h, "program.transport.exportFrame"));
    assert!(!has(&h, "program.transport.editor"), "no button editor");
    assert!(has(&h, "program.transport.media.toggleProxies"), "the proxy toggle stays");
    assert!(has(&h, "program.transport.playback.toggle"), "play stays");
    assert!(has(&h, "program.transport.playhead.stepBack"));
    assert!(has(&h, "program.transport.playhead.stepForward"));
    assert!(h.state().ui_error.is_none(), "{:?}", h.state().ui_error);
}

#[test]
fn the_minimal_interface_closes_history_essential_graphics_and_text() {
    let h = harness(true);
    for panel in [PanelKind::History, PanelKind::EssentialGraphics, PanelKind::Text, PanelKind::Tools] {
        assert!(!h.state().ui.dock.contains(panel), "{panel:?} still docked");
    }
    for panel in [PanelKind::Program, PanelKind::Timeline, PanelKind::Project, PanelKind::Properties] {
        assert!(h.state().ui.dock.contains(panel), "{panel:?} missing");
    }
}

#[test]
fn the_minimal_timeline_has_a_thin_video_lane_tall_audio_lanes_and_slim_headers() {
    let h = harness(true);
    assert!(!has(&h, "timeline.track.V1.sourcePatch"));
    assert!(!has(&h, "timeline.track.V1.target"));
    assert!(!has(&h, "timeline.track.V1.syncLock"));
    assert!(has(&h, "timeline.track.V1.locked"), "lock stays");
    assert!(has(&h, "timeline.track.V1.enabled"), "eye stays");
    assert!(has(&h, "timeline.track.A1.muted"), "mute stays");
    assert!(has(&h, "timeline.track.A1.solo"), "solo stays");
    let tl = &h.state().ui.timeline;
    assert!(tl.header_w < 204.0, "narrower header column: {}", tl.header_w);
    assert!(tl.video_track_h < 40.0, "thin video lane: {}", tl.video_track_h);
    assert!(tl.audio_track_h > 90.0, "tall audio lanes for the waveform: {}", tl.audio_track_h);
}
