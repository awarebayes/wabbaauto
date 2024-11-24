pub const JS_INJECTION: &str = r###"
$("#slowDownloadButton").off("click");
$(function () {
    const isNmmDownload = false;

    $('#slowDownloadButton').click(function () {
        const downloadUrl = $(this).data('download-url');
        $('.subheader, .table').hide();

        $('.donation-wrapper').show();

        startDownload(downloadUrl);
    });

    $('#startDownloadButton').click(function () {
        const downloadUrl = $(this).data('download-url');
        startDownload(downloadUrl);
    });

    function startDownload(downloadUrl) {
        $('.donation-wrapper').show();
        if (isNmmDownload) {
            download(downloadUrl);
        } else {
            $.ajax(
                {
                    type: "POST",
                    url: "/Core/Libs/Common/Managers/Downloads?GenerateDownloadUrl",
                    data: {
                        fid: window.file_id,
                        game_id: window.current_game_id,
                    },
                    success: function (data) {
                        if (data && data.url) {
                            download(data.url);
                        } else {
                            setError();
                        }
                    },
                    error: function () {
                        setError();
                    }
                }
            );
        }
    }

    function download(downloadUrl) {
        window.nexusDataLayer.push({
            event: 'mod_download',
            file_id,
            download_method: isNmmDownload ? 'Vortex' : 'Manual',
        });
        window.downloadUrl = downloadUrl;
        $('.donation-wrapper > p').html('<p>Your download has started</p><p>If you are having trouble, <a href="' + downloadUrl + '" id="downloadUrl">click here</a> to download manually</p>');
    }

    function setError() {
        console.log('An error occurred');
        $('.donation-wrapper > p').html('<p>Unfortunately an error occurred while downloading this file</p><p>Please try again later or contact support</p>');
    }
});
"###;
